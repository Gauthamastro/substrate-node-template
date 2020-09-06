use sp_arithmetic::FixedU128;
use sp_std::vec::Vec;
use codec::{Encode, Decode};
#[derive(Encode, Decode, PartialEq)]
pub struct MarketData {
    current_block_num: u64,
    closing_ask: FixedU128,
    closing_bid: FixedU128,
    volume: FixedU128,
}
#[derive(Encode, Decode, PartialEq)]
pub struct PriceLevelData {
    pub(crate) price_level: FixedU128,
    pub(crate) amount: FixedU128,
}

#[derive(Encode, Decode, PartialEq)]
pub struct OrderBookApi {
    pub(crate) bids: Vec<PriceLevelData>,
    pub(crate) asks: Vec<PriceLevelData>,
    pub(crate) market_data: Vec<MarketData>,
    pub(crate) enabled: bool,
}