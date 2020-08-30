use substrate_fixed::types::U32F32;
use codec::{Encode, Decode};
use sp_std::cmp::Ordering;
use sp_std::vec::Vec;
use sp_std::collections::vec_deque::VecDeque;
use crate::binary_heap::BinaryHeap;
use sp_std::collections::btree_map;

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
pub struct Order<AccountId, BlockNumber> {
    id: Vec<u8>,
    order_type: OrderType,
    price: U32F32,
    quantity: U32F32,
    origin: AccountId,
    expiry: BlockNumber,
}

#[derive(Encode, Decode, Clone, Debug, Default)]
pub struct PriceLevel<AccountId, BlockNumber> {
    price_level: U32F32,
    queue: VecDeque<Order<AccountId, BlockNumber>>,
}

impl<AccountId, BlockNumber> Ord for PriceLevel<AccountId, BlockNumber> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price_level.cmp(&other.price_level)
    }
}

impl<AccountId, BlockNumber> PartialOrd for PriceLevel<AccountId, BlockNumber> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<AccountId, BlockNumber> PartialEq for PriceLevel<AccountId, BlockNumber> {
    fn eq(&self, other: &Self) -> bool {
        self.price_level == other.price_level
    }
}

impl<AccountId, BlockNumber> Eq for PriceLevel<AccountId, BlockNumber> {}

#[derive(Encode, Decode, Default, Clone, PartialEq, Debug)]
pub struct OrderBook<AccountId, BlockNumber, AssetId> {
    id: TradingPair,
    // notation: String BTC/ETH
    trading_asset: AssetId,
    // BTC -- AssetId from GenericAsset
    quote_asset: AssetId,
    // ETH -- AssetId from GenericAsset
    nonce: u64,
    orders: btree_map::BTreeMap<Vec<u8>, Order<AccountId, BlockNumber>>,
    advanced_bid_orders: BinaryHeap<PriceLevel<AccountId, BlockNumber>>,
    advanced_ask_orders: BinaryHeap<PriceLevel<AccountId, BlockNumber>>,
    bids: BinaryHeap<PriceLevel<AccountId, BlockNumber>>,
    asks: BinaryHeap<PriceLevel<AccountId, BlockNumber>>,
    market_data: Vec<MarketData<BlockNumber>>,
    enabled: bool,
}

impl<AccountId, BlockNumber, AssetId> OrderBook<AccountId, BlockNumber, AssetId> {
    pub fn get_trading_asset(&self) -> &AssetId {
        return &self.trading_asset;
    }

    pub fn get_quote_asset(&self) -> &AssetId {
        return &self.quote_asset;
    }

    pub fn get_orders(&self) -> &btree_map::BTreeMap<Vec<u8>, Order<AccountId, BlockNumber>> {
        return &self.orders;
    }
}

#[derive(Encode, Decode, Clone, Debug, Default)]
pub struct MarketData<BlockNumber> {
    current_block: BlockNumber,
    opening_bid: U32F32,
    opening_ask: U32F32,
    closing_bid: U32F32,
    closing_ask: U32F32,
    volume: U32F32,
}


// FIXME(The given implementation is not correct and needs to be fixed later)
impl<BlockNumber> Ord for MarketData<BlockNumber> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<BlockNumber> PartialOrd for MarketData<BlockNumber> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<BlockNumber> PartialEq for MarketData<BlockNumber> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<BlockNumber> Eq for MarketData<BlockNumber> {}


impl<AccountId, BlockNumber> Ord for BinaryHeap<PriceLevel<AccountId, BlockNumber>> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<AccountId, BlockNumber> PartialOrd for BinaryHeap<PriceLevel<AccountId, BlockNumber>> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<AccountId, BlockNumber> PartialEq for BinaryHeap<PriceLevel<AccountId, BlockNumber>> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<AccountId, BlockNumber> Eq for BinaryHeap<PriceLevel<AccountId, BlockNumber>> {}



