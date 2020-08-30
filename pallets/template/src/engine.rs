use substrate_fixed::types::U32F32;
use codec::{Encode, Decode};
use sp_std::cmp::Ordering;
use sp_std::vec::Vec;
use sp_std::collections::vec_deque::VecDeque;
use crate::binary_heap::BinaryHeap;

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
pub struct Order<AccountId,Hash,BlockNumber> {
    id: Hash,
    order_type: OrderType,
    price: U32F32,
    quantity: U32F32,
    origin: AccountId,
    expiry: BlockNumber
}

#[derive(Encode, Decode, Clone, Debug, Default)]
pub struct PriceLevel<AccountId,Hash,BlockNumber> {
    price_level: U32F32,
    queue: VecDeque<Order<AccountId,Hash,BlockNumber>>,
}

impl<AccountId,Hash,BlockNumber>  Ord for PriceLevel<AccountId,Hash,BlockNumber> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price_level.cmp(&other.price_level)
    }
}

impl<AccountId,Hash,BlockNumber>  PartialOrd for PriceLevel<AccountId,Hash,BlockNumber> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<AccountId,Hash,BlockNumber>  PartialEq for PriceLevel<AccountId,Hash,BlockNumber> {
    fn eq(&self, other: &Self) -> bool {
        self.price_level == other.price_level
    }
}

impl<AccountId,Hash,BlockNumber>  Eq for PriceLevel<AccountId,Hash,BlockNumber> {}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct OrderBook<AccountId,Hash,BlockNumber,AssetId> {
    id: TradingPair,
    // notation: String BTC/ETH
    trading_asset: AssetId, // BTC -- AssetId from GenericAsset
    quote_asset: AssetId, // ETH -- AssetId from GenericAsset
    nonce: u64,
    order_ids: VecDeque<Hash>,
    orders: VecDeque<Order<AccountId,Hash,BlockNumber>>,
    advanced_bid_orders: BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>>,
    advanced_ask_orders: BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>>,
    bids: BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>>,
    asks: BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>>,
    market_data: Vec<MarketData<BlockNumber>>
}

#[derive(Encode, Decode, Clone, Debug, Default)]
pub struct MarketData<BlockNumber>{
    current_block: BlockNumber,
    opening_bid: U32F32,
    opening_ask: U32F32,
    closing_bid: U32F32,
    closing_ask: U32F32,
    volume: U32F32
}


// FIXME(The given implementation is not correct and needs to be fixed later)
impl<BlockNumber>  Ord for MarketData<BlockNumber> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<BlockNumber>  PartialOrd for MarketData<BlockNumber> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<BlockNumber>  PartialEq for MarketData<BlockNumber> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<BlockNumber>  Eq for MarketData<BlockNumber> {}


impl<AccountId,Hash,BlockNumber> Ord for BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<AccountId,Hash,BlockNumber> PartialOrd for BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<AccountId,Hash,BlockNumber> PartialEq for BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<AccountId,Hash,BlockNumber> Eq for BinaryHeap<PriceLevel<AccountId,Hash,BlockNumber>> {}



