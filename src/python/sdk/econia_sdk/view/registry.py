from aptos_sdk.account_address import AccountAddress
from typing import Optional
from econia_sdk.lib import EconiaViewer

def get_MAX_CHARACTERS_GENERIC(view: EconiaViewer) -> int:
    returns = view.get_returns(
        "registry",
        "get_MAX_CHARACTERS_GENERIC",
    )
    return int(returns[0])

def get_MIN_CHARACTERS_GENERIC(view: EconiaViewer) -> int:
    returns = view.get_returns(
        "registry",
        "get_MIN_CHARACTERS_GENERIC",
    )
    return int(returns[0])

def get_NO_CUSTODIAN(view: EconiaViewer) -> int:
    returns = view.get_returns(
        "registry",
        "get_NO_CUSTODIAN",
    )
    return int(returns[0])

def get_NO_UNDERWRITER(view: EconiaViewer) -> int:
    returns = view.get_returns(
        "registry",
        "get_NO_UNDERWRITER",
    )
    return int(returns[0])

def get_market_counts(view: EconiaViewer) -> dict:
    returns = view.get_returns(
      "registry",
      "get_market_counts",
    )
    value = returns[0]
    return {
        "n_markets": int(value["n_markets"]),
        "n_recognized_markets": int(value["n_recognized_markets"])
    }

def get_market_info(view: EconiaViewer, market_id: int) -> dict:
    returns = view.get_returns(
        "registry",
        "get_market_info",
        [],
        [str(market_id)]
    )
    value = returns[0]
    return {
        "base_name_generic": value["base_name_generic"],
        "base_type": {
            "module_name": value["base_type"]["module_name"],
            "package_address": AccountAddress.from_hex(value["base_type"]["package_address"]),
            "type_name": value["base_type"]["type_name"],
        },
        "is_recognized": bool(value["is_recognized"]),
        "lot_size": int(value["lot_size"]), # subunits of base
        "market_id": int(value["market_id"]),
        "min_size": int(value["min_size"]),
        "quote_type": {
            "module_name": value["quote_type"]["module_name"],
            "package_address": AccountAddress.from_hex(value["quote_type"]["package_address"]),
            "type_name": value["quote_type"]["type_name"],
        },
        "tick_size": int(value["tick_size"]),
        "underwriter_id": int(value["underwriter_id"]),
    }

def get_recognized_market_id_base_coin(
    view: EconiaViewer,
    base_coin_type: str,
    quote_coin_type: str,
) -> int:
    returns = view.get_returns(
        "registry",
        "get_recognized_market_id_base_coin",
        [base_coin_type, quote_coin_type],
    )
    return int(returns[0])

def get_recognized_market_id_base_generic(
    view: EconiaViewer,
    quote_coin_type: str,
) -> int:
    returns = view.get_returns(
        "registry",
        "get_recognized_market_id_base_generic",
        [quote_coin_type],
    )
    return int(returns[0])

def has_recognized_market_base_coin_by_type(
    view: EconiaViewer,
    base_coin_type: str,
    quote_coin_type: str,
) -> bool:
    returns = view.get_returns(
        "registry",
        "has_recognized_market_base_coin_by_type",
        [base_coin_type, quote_coin_type],
    )
    return bool(returns[0])

def has_recognized_market_base_generic_by_type(
    view: EconiaViewer,
    quote_coin_type: str,
    base_name_generic: str,
) -> bool:
    returns = view.get_returns(
        "registry",
        "has_recognized_market_base_generic_by_type",
        [quote_coin_type],
        [base_name_generic]
    )
    return bool(returns[0])

def get_market_id_base_coin(
    view: EconiaViewer,
    base_coin_type: str,
    quote_coin_type: str,
    lot_size: int,
    tick_size: int,
    min_size: int,
) -> Optional[int]: # might be None
    returns = view.get_returns(
        "registry",
        "get_market_id_base_coin",
        [base_coin_type, quote_coin_type],
        [
            str(lot_size),
            str(tick_size),
            str(min_size),
        ]
    )
    opt_val = returns[0]['vec']
    if len(opt_val) == 0:
        return None
    else:
        return int(opt_val[0])

def get_market_id_base_generic(
        view: EconiaViewer,
        quote_type: str,
        base_name_generic: str,
        lot_size: int,
        tick_size: int,
        min_size: int,
        underwriter_id: int = 0,
) -> Optional[int]:
    returns = view.get_returns(
        "registry",
        "get_market_id_base_generic",
        [quote_type],
        [
            base_name_generic,
            str(lot_size),
            str(tick_size),
            str(min_size),
            str(underwriter_id)
        ]
    )
    opt_val = returns[0]['vec']
    if len(opt_val) == 0:
        return None
    else:
        return int(opt_val[0])

