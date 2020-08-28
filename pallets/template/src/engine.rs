use substrate_fixed::types::U32F32;
use codec::{Encode, Decode};
use sp_std::vec::Vec;
use binary_heap_plus::{BinaryHeap, MaxComparator, MinComparator};
use sp_std::cmp::Ordering;
use sp_std::collections::vec_deque::VecDeque;

pub type OrderId = Vec<u8>;
pub type BlockNumber = u32;

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
pub struct Order {
    id: OrderId,
    order_type: OrderType,
    price: U32F32,
    quantity: U32F32,
}

pub struct PriceLevel {
    price_level: U32F32,
    queue: VecDeque<Order>,
}

impl Ord for PriceLevel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price_level.cmp(&other.price_level)
    }
}

impl PartialOrd for PriceLevel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PriceLevel {
    fn eq(&self, other: &Self) -> bool {
        self.price_level == other.price_level
    }
}

impl Eq for PriceLevel {}

pub struct OrderBook {
    id: u32,
    // notation: String BTC/ETH
    trading_asset: u32, // BTC -- AssetId from GenericAsset
    quote_asset: u32, // ETH -- AssetId from GenericAsset
    nonce: u64,
    order_ids: VecDeque<OrderId>,
    orders: VecDeque<Order>,
    advanced_bid_orders: BinaryHeap<PriceLevel, MaxComparator>,
    advanced_ask_orders: BinaryHeap<PriceLevel, MinComparator>,
    expiring_orders: BinaryHeap<BlockNumber, MinComparator>,
    bids: BinaryHeap<PriceLevel, MaxComparator>,
    asks: BinaryHeap<PriceLevel, MinComparator>,
}