use std::{collections::HashSet, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use bigdecimal::ToPrimitive;
use chrono::{DateTime, Utc};
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS, Transport};
use serde::{Deserialize, Serialize};
use sqlx_postgres::{PgListener, PgPool};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize)]
struct PlaceLimitOrderNotif {
    txn_version: u128,
    event_idx: u128,
    time: DateTime<Utc>,
    market_id: u128,
    user: String,
    custodian_id: u128,
    order_id: u128,
    side: bool,
    integrator: String,
    initial_size: u128,
    price: u128,
    restriction: i16,
    self_match_behavior: i16,
    size: u128,
}
#[derive(Serialize, Deserialize)]
struct PlaceMarketOrderNotif {
    txn_version: u128,
    event_idx: u128,
    time: DateTime<Utc>,
    market_id: u128,
    user: String,
    custodian_id: u128,
    order_id: u128,
    direction: bool,
    integrator: String,
    self_match_behavior: i16,
    size: u128,
}
#[derive(Serialize, Deserialize)]
struct PlaceSwapOrderNotif {
    txn_version: u128,
    event_idx: u128,
    time: DateTime<Utc>,
    market_id: u128,
    order_id: u128,
    direction: bool,
    signing_account: String,
    integrator: String,
    min_base: u128,
    max_base: u128,
    min_quote: u128,
    max_quote: u128,
    limit_price: u128,
}
#[derive(Serialize, Deserialize)]
struct ChangeOrderSizeNotif {
    txn_version: u128,
    event_idx: u128,
    time: DateTime<Utc>,
    market_id: u128,
    user: String,
    custodian_id: u128,
    order_id: u128,
    side: bool,
    new_size: u128,
}
#[derive(Serialize, Deserialize)]
struct CancelOrderNotif {
    txn_version: u128,
    event_idx: u128,
    time: DateTime<Utc>,
    market_id: u128,
    user: String,
    custodian_id: u128,
    order_id: u128,
    reason: i16,
}
#[derive(Serialize, Deserialize)]
struct FillNotif {
    txn_version: u128,
    event_idx: u128,
    emit_address: String,
    time: DateTime<Utc>,
    maker_address: String,
    maker_custodian_id: u128,
    maker_order_id: u128,
    maker_side: bool,
    market_id: u128,
    price: u128,
    sequence_number_for_trade: u128,
    size: u128,
    taker_address: String,
    taker_custodian_id: u128,
    taker_order_id: u128,
    taker_quote_fees_paid: u128,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mqtt_url = std::env::var("MQTT_URL")?;
    let mqtt_password = std::env::var("MQTT_PASSWORD")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let mqtt_price_levels = std::env::var("MQTT_PRICE_LEVELS")?;

    let mut mqttoptions = MqttOptions::parse_url(format!("{mqtt_url}/?client_id=mqtt_publisher")).unwrap();
    mqttoptions.set_credentials("mqtt_publisher", mqtt_password);
    mqttoptions.set_transport(Transport::Tcp);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

    let mqtt_client = Arc::new(RwLock::new(client));

    let pnl = postgres_notif_loop(&db_url, mqtt_client.clone());
    let epl = eventpoll_loop(eventloop);
    if mqtt_price_levels == "yes" {
        let pll = price_level_loop(&db_url, mqtt_client);
        tokio::try_join!(epl, pnl, pll)?;
    } else {
        tokio::try_join!(epl, pnl)?;
    }


    Ok(())
}

async fn eventpoll_loop(mut eventloop: EventLoop) -> Result<()> {
    loop {
        eventloop.poll().await.unwrap();
    }
}

#[derive(Serialize)]
struct PriceLevel {
    price: u128,
    amount: u128,
}

async fn price_level_loop(db_url: &str, mqtt_client: Arc<RwLock<AsyncClient>>) -> Result<()> {
    let pool = PgPool::connect(&db_url).await?;

    loop {
        tokio::time::sleep(Duration::from_millis(50)).await;
        let data = sqlx::query!(r#"
            WITH levels AS (
                SELECT
                    market_id,
                    direction::text,
                    price,
                    SUM(remaining_size) AS total_size
                FROM
                    aggregator.user_history
                WHERE
                    order_status = 'open'
                GROUP BY
                    market_id,
                    direction,
                    price
                ORDER BY
                    market_id,
                    direction,
                    CASE
                        WHEN direction = 'ask' THEN price
                        ELSE -1 * price
                    END
            ),
            numbered_levels AS (
                SELECT
                    *,
                    row_number() OVER (PARTITION BY market_id, direction) AS level
                FROM
                    levels
            )
            SELECT * FROM numbered_levels WHERE level <= 10;
        "#).fetch_all(&pool).await?;
        let mqtt_client = mqtt_client.read().await;
        for row in data {
            let topic = format!("levels/{}/{}/{}", row.market_id, row.direction.ok_or(anyhow!("direction is None"))?, row.level.ok_or(anyhow!("level is None"))?);
            let payload = PriceLevel {
                price: row.price.ok_or(anyhow!("price is None"))?.to_u128().ok_or(anyhow!("price is too big"))?,
                amount: row.total_size.ok_or(anyhow!("total_size is None"))?.to_u128().ok_or(anyhow!("total_size is too big"))?,
            };
            mqtt_client.publish(topic, QoS::AtLeastOnce, false, serde_json::to_string(&payload)?).await?;
        }
    }
}

async fn postgres_notif_loop(db_url: &str, mqtt_client: Arc<RwLock<AsyncClient>>) -> anyhow::Result<()> {
    let mut listener = PgListener::connect(&db_url).await?;
    let channels = vec![
        "place_limit_order",
        "place_market_order",
        "place_swap_order",
        "change_order_size",
        "cancel_order",
        "fill",
    ];
    listener.listen_all(channels).await?;
    let mut emitted_fills: HashSet<(
        u128,
        u128,
        u128,
    )> = Default::default();
    loop {
        let notification = listener.recv().await?;
        let mqtt_client = mqtt_client.read().await;
        match notification.channel() {
            "place_limit_order" => {
                let data: PlaceLimitOrderNotif = serde_json::from_str(notification.payload())?;
                mqtt_client
                    .publish(
                        format!(
                            "place_limit_order/{}/{}/{}/{}",
                            data.market_id, data.user, data.custodian_id, data.integrator
                        ),
                        QoS::AtLeastOnce,
                        false,
                        serde_json::to_string(&data)?,
                    )
                    .await?;
            }
            "place_market_order" => {
                let data: PlaceMarketOrderNotif = serde_json::from_str(notification.payload())?;
                mqtt_client
                    .publish(
                        format!(
                            "place_market_order/{}/{}/{}/{}",
                            data.market_id, data.user, data.custodian_id, data.integrator
                        ),
                        QoS::AtLeastOnce,
                        false,
                        serde_json::to_string(&data)?,
                    )
                    .await?;
            }
            "place_swap_order" => {
                let data: PlaceSwapOrderNotif = serde_json::from_str(notification.payload())?;
                mqtt_client
                    .publish(
                        format!(
                            "place_swap_order/{}/{}/{}",
                            data.market_id, data.integrator, data.signing_account
                        ),
                        QoS::AtLeastOnce,
                        false,
                        serde_json::to_string(&data)?,
                    )
                    .await?;
            }
            "change_order_size" => {
                let data: ChangeOrderSizeNotif = serde_json::from_str(notification.payload())?;
                mqtt_client
                    .publish(
                        format!(
                            "change_order_size/{}/{}/{}",
                            data.market_id, data.user, data.custodian_id
                        ),
                        QoS::AtLeastOnce,
                        false,
                        serde_json::to_string(&data)?,
                    )
                    .await?;
            }
            "cancel_order" => {
                let data: CancelOrderNotif = serde_json::from_str(notification.payload())?;
                mqtt_client
                    .publish(
                        format!(
                            "cancel_order/{}/{}/{}",
                            data.market_id, data.user, data.custodian_id
                        ),
                        QoS::AtLeastOnce,
                        false,
                        serde_json::to_string(&data)?,
                    )
                    .await?;
            }
            "fill" => {
                let data: FillNotif = serde_json::from_str(notification.payload())?;
                if !emitted_fills.remove(&(
                    data.market_id.clone(),
                    data.taker_order_id.clone(),
                    data.sequence_number_for_trade.clone(),
                )) {
                    mqtt_client
                        .publish(
                            format!(
                                "fill/{}/{}/{}",
                                data.market_id, data.maker_address, data.maker_custodian_id
                            ),
                            QoS::AtLeastOnce,
                            false,
                            serde_json::to_string(&data)?,
                        )
                        .await?;
                    mqtt_client
                        .publish(
                            format!(
                                "fill/{}/{}/{}",
                                data.market_id, data.taker_address, data.taker_custodian_id
                            ),
                            QoS::AtLeastOnce,
                            false,
                            serde_json::to_string(&data)?,
                        )
                        .await?;
                    emitted_fills.insert((
                        data.market_id,
                        data.taker_order_id,
                        data.sequence_number_for_trade,
                    ));
                }
            }
            _ => {}
        }
    }
}