import BigNumber from "bignumber.js";
import { useEffect, useMemo, useRef } from "react";

import { useOrderEntry } from "@/contexts/OrderEntryContext";
import { type ApiMarket } from "@/types/api";
// import { type Precision } from "@/types/global";
import { type Orderbook, type PriceLevel } from "@/types/global";
import { toDecimalPrice, toDecimalSize } from "@/utils/econia";
import { averageOrOtherPriceLevel } from "@/utils/formatter";

// const precisionOptions: Precision[] = [
//   "0.01",
//   "0.05",
//   "0.1",
//   "0.5",
//   "1",
//   "2.5",
//   "5",
//   "10",
// ];

const Row: React.FC<{
  level: PriceLevel;
  type: "bid" | "ask";
  highestSize: number;
  marketData: ApiMarket;
}> = ({ level, type, highestSize, marketData }) => {
  const { setType, setPrice } = useOrderEntry();
  const price = toDecimalPrice({
    price: new BigNumber(level.price),
    lotSize: BigNumber(marketData.lot_size),
    tickSize: BigNumber(marketData.tick_size),
    baseCoinDecimals: BigNumber(marketData.base?.decimals || 0),
    quoteCoinDecimals: BigNumber(marketData.quote?.decimals || 0),
  }).toNumber();

  const size = toDecimalSize({
    size: new BigNumber(level.size),
    lotSize: BigNumber(marketData.lot_size),
    baseCoinDecimals: BigNumber(marketData.base?.decimals || 0),
  });

  return (
    <div
      className="relative flex h-6 items-center justify-between py-[1px] hover:ring-1 hover:ring-neutral-600"
      onClick={() => {
        setType(type === "ask" ? "buy" : "sell");
        setPrice(price.toString());
      }}
    >
      <div
        className={`z-10 ml-4 text-right font-roboto-mono text-xs ${
          type === "ask" ? "text-red" : "text-green"
        }`}
      >
        {price}
      </div>
      <div className="z-10 mr-4 py-0.5 font-roboto-mono text-xs text-white">
        {size.toPrecision(4)}
      </div>
      <div
        className={`absolute right-0 z-0 h-full opacity-30 ${
          type === "ask" ? "bg-red" : "bg-green"
        }`}
        // dynamic taillwind?

        style={{ width: `${(100 * level.size) / highestSize}%` }}
      ></div>
    </div>
  );
};

export function OrderbookTable({
  marketData,
  data,
  isFetching,
  isLoading,
}: {
  marketData: ApiMarket;
  data: Orderbook | undefined;
  isFetching: boolean;
  isLoading: boolean;
}) {
  // const [precision, setPrecision] = useState<Precision>(precisionOptions[0]);

  const centerRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    centerRef.current?.scrollIntoView({
      behavior: "smooth",
      block: "center",
    });
  }, [isFetching]);

  const midPrice: PriceLevel | undefined = useMemo(() => {
    if (data == null) {
      return undefined;
    }
    return averageOrOtherPriceLevel(
      data.asks ? data.asks[0] : undefined,
      data.bids ? data.bids[0] : undefined,
    );
  }, [data]);

  const highestSize = useMemo(() => {
    if (data == null) {
      return 0;
    }
    return Math.max(
      ...data.asks.map((order) => order.size),
      ...data.bids.map((order) => order.size),
    );
  }, [data]);

  if (isLoading) {
    return (
      <div className="flex h-full flex-col items-center justify-center text-sm font-light uppercase text-neutral-500">
        Loading...
      </div>
    );
  }

  return (
    <div className="flex grow flex-col">
      {/* title row */}
      <div className="border-b border-neutral-600 px-4 py-3">
        <div className="flex justify-between">
          <p className="font-jost text-sm font-bold text-white">Orderbook</p>
          {/* select */}
          {/* TODO: SHOW WHEN API IS UP */}
          {/* <Listbox value={precision} onChange={setPrecision}>
            <div className="relative z-30 min-h-[30px] border border-neutral-600 py-[4px] pl-[8px] pr-[4px] text-[8px]/[18px]">
              <Listbox.Button className="flex min-w-[48px] justify-between font-roboto-mono text-neutral-300">
                {precision}
                <ChevronDownIcon className="my-auto ml-1 h-[10px] w-[10px] text-neutral-500" />
              </Listbox.Button>
              <Listbox.Options className="absolute left-0 top-[20px] mt-2 w-full bg-black shadow ring-1 ring-neutral-600">
                {precisionOptions.map((precisionOption) => (
                  <Listbox.Option
                    key={precisionOption}
                    value={precisionOption}
                    className={`weight-300  box-border flex min-h-[30px] cursor-pointer justify-between py-2 pl-[11px] font-roboto-mono text-neutral-300 hover:bg-neutral-800  hover:outline hover:outline-1 hover:outline-neutral-600`}
                  >
                    {precisionOption}
                    {precision === precisionOption && (
                      <CheckIcon className="my-auto ml-1 mr-2 h-4 w-4 text-white" />
                    )}
                  </Listbox.Option>
                ))}
              </Listbox.Options>
            </div>
          </Listbox> */}
        </div>
        <div className="mt-3 flex justify-between">
          <p className="font-roboto-mono text-xs text-neutral-500">
            PRICE ({marketData.quote.symbol})
          </p>
          <p className="font-roboto-mono text-xs text-neutral-500">
            Size ({marketData.base?.symbol})
          </p>
        </div>
      </div>
      {/* bids ask spread scrollable container */}
      <div className="scrollbar-none relative grow overflow-y-auto">
        <div className="absolute w-full">
          {/* ASK */}
          {data?.asks.map((level) => (
            <Row
              level={level}
              type={"ask"}
              key={`ask-${level.price}-${level.size}`}
              highestSize={highestSize}
              marketData={marketData}
            />
          ))}
          {/* SPREAD */}
          <div
            className="flex items-center justify-between border-y border-neutral-600"
            ref={centerRef}
          >
            <div className="z-10 ml-4 text-right font-roboto-mono text-xs text-white">
              {toDecimalPrice({
                price: new BigNumber(midPrice?.price || 0),
                lotSize: BigNumber(marketData.lot_size),
                tickSize: BigNumber(marketData.tick_size),
                baseCoinDecimals: BigNumber(marketData.base?.decimals || 0),
                quoteCoinDecimals: BigNumber(marketData.quote?.decimals || 0),
              }).toNumber()}
            </div>
            <div className="mr-4 font-roboto-mono text-white">
              {midPrice?.size || "-"}
            </div>
          </div>
          {/* BID */}
          {data?.bids.map((level) => (
            <Row
              level={level}
              type={"bid"}
              key={`bid-${level.price}-${level.size}`}
              highestSize={highestSize}
              marketData={marketData}
            />
          ))}
        </div>
      </div>
    </div>
  );
}
