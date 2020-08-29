use substrate_fixed::types::U32F32;
use codec::{Encode, Decode};
use sp_std::vec::Vec;
use sp_std::cmp::Ordering;
use sp_std::collections::vec_deque::VecDeque;
use crate::binary_heap::BinaryHeap;

pub type OrderId = Vec<u8>;
pub type BlockNumber = u32;
pub type TradingPair = u32;

#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub enum OrderType {
    BidLimit,
    BidMarket,
    BidStopLimit,
    BidStopMarket,
    BidStopLoss,
    BidFillKill,

    PostOnly,

    AskLimit,
    AskMarket,
    AskStopLimit,
    AskStopMarket,
    AskStopLoss,
    AskFillKill,

    None,
}

impl Default for OrderType {
    fn default() -> Self { OrderType::None }
}


#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct Order<AccountId> {
    id: OrderId,
    order_type: OrderType,
    price: U32F32,
    quantity: U32F32,
    origin: AccountId
}

#[derive(Encode, Decode, Clone, Debug, Default)]
pub struct PriceLevel<AccountId> {
    price_level: U32F32,
    queue: VecDeque<Order<AccountId>>,
}

impl<AccountId> Ord for PriceLevel<AccountId> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price_level.cmp(&other.price_level)
    }
}

impl<AccountId> PartialOrd for PriceLevel<AccountId> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<AccountId> PartialEq for PriceLevel<AccountId> {
    fn eq(&self, other: &Self) -> bool {
        self.price_level == other.price_level
    }
}

impl<AccountId> Eq for PriceLevel<AccountId> {}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct OrderBook<AccountId> {
    id: TradingPair,
    // notation: String BTC/ETH
    trading_asset: u32, // BTC -- AssetId from GenericAsset
    quote_asset: u32, // ETH -- AssetId from GenericAsset
    nonce: u64,
    order_ids: VecDeque<OrderId>,
    orders: VecDeque<Order<AccountId>>,
    advanced_bid_orders: BinaryHeap<PriceLevel<AccountId>>,
    advanced_ask_orders: BinaryHeap<PriceLevel<AccountId>>,
    expiring_orders: BinaryHeap<BlockNumber>,
    bids: BinaryHeap<PriceLevel<AccountId>>,
    asks: BinaryHeap<PriceLevel<AccountId>>,
}


impl<AccountId> Ord for BinaryHeap<PriceLevel<AccountId>> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<AccountId> PartialOrd for BinaryHeap<PriceLevel<AccountId>> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<AccountId> PartialEq for BinaryHeap<PriceLevel<AccountId>> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<AccountId> Eq for BinaryHeap<PriceLevel<AccountId>> {}

impl Ord for BinaryHeap<BlockNumber> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl PartialOrd for BinaryHeap<BlockNumber> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl PartialEq for BinaryHeap<BlockNumber> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for BinaryHeap<BlockNumber> {}




