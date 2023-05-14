import { Tab } from "@headlessui/react";
import React, { useState } from "react";

import { type Side } from "@/types/global";
import { ApiMarket } from "@/types/api";
import { LimitOrderEntry } from "./LimitOrderEntry";
import { MarketOrderEntry } from "./MarketOrderEntry";

export const OrderEntry: React.FC<{ marketData: ApiMarket }> = ({
  marketData,
}) => {
  const [side, setSide] = useState<Side>("buy");
  return (
    <div>
      <div className="m-4 flex gap-2">
        <button
          onClick={() => setSide("buy")}
          className={`w-full border py-3 font-jost ${
            side == "buy"
              ? "border-green border-opacity-80 text-green"
              : "border-neutral-600 bg-neutral-700 text-neutral-600"
          }`}
        >
          Buy
        </button>
        <button
          onClick={() => setSide("sell")}
          className={`w-full border font-jost ${
            side == "sell"
              ? "border-red border-opacity-80 text-red"
              : "border-neutral-600 bg-neutral-700 text-neutral-600"
          }`}
        >
          Sell
        </button>
      </div>
      <Tab.Group>
        <Tab.List className="my-5 flex justify-evenly">
          <Tab className="font-roboto-mono text-sm font-light uppercase outline-none ui-selected:text-white ui-not-selected:text-neutral-500">
            Limit
          </Tab>
          <Tab className="font-roboto-mono text-sm font-light uppercase outline-none ui-selected:text-white ui-not-selected:text-neutral-500">
            Market
          </Tab>
          {/* <Tab className="font-roboto-mono text-sm font-light uppercase outline-none ui-selected:text-white ui-not-selected:text-neutral-500">
            Stop Limit
          </Tab> */}
        </Tab.List>
        <Tab.Panels>
          <Tab.Panel>
            <LimitOrderEntry marketData={marketData} side={side} />
          </Tab.Panel>
          <Tab.Panel className="px-2 font-jost text-white">
            <MarketOrderEntry marketData={marketData} side={side} />
          </Tab.Panel>
          {/* <Tab.Panel className="px-2 font-jost text-white">
            Stop Limit Order Entry
          </Tab.Panel> */}
        </Tab.Panels>
      </Tab.Group>
    </div>
  );
};
