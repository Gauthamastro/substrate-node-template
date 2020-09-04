#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch};
use frame_support::sp_runtime::offchain::storage_lock::BlockNumberProvider;
use frame_system::ensure_signed;
use pallet_generic_asset::AssetIdProvider;
use sp_arithmetic::{FixedPointNumber, FixedU128};
use sp_arithmetic::traits::{CheckedDiv, CheckedMul, CheckedSub, UniqueSaturatedFrom};
use sp_std::collections::btree_map;
use sp_std::collections::vec_deque::VecDeque;
use sp_std::convert::TryInto;
use sp_std::str;
use sp_std::vec::Vec;

use crate::engine::{Order, OrderBook};

pub mod binary_heap;
mod engine;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
/// pallet_generic_asset::Trait bounds this DEX pallet with pallet_generic_asset. DEX is available
/// only for runtimes that also install pallet_generic_asset.
pub trait Trait: frame_system::Trait + pallet_generic_asset::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId,
	 Balance = <T as pallet_generic_asset::Trait>::Balance,
	  BlockNumber = <T as frame_system::Trait>::BlockNumber{
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		TradeAmount(Balance, FixedU128, AccountId),
		/// Not enough asset free balance for placing the trade
		InsufficientAssetBalance(FixedU128),
		/// Order contains a duplicate orderId of another active order
		DuplicateOrderId(Vec<u8>),
		/// Order type of Order is None
		OrderTypeIsNone,
		/// Price and Quantity cannot be zero
		PriceOrQuanitityIsZero,
		/// Invalid TradingPair Id
		TradingPairNotFound(u32),
		/// Same Assets cannot be traded
		SameAssetIdsError(u32,u32),
		/// Zero Balances in either one or both Assets
		NoBalanceOfAssets(u128,u128),
		/// When a new TradingPair is created
		TradingPairCreated(u32),
		/// New Order created
		NewOrderCreated(Vec<u8>,engine::OrderType,FixedU128,FixedU128,AccountId,BlockNumber),
		/// Contains market state about current block.
		/// Order: tradingPair,blockNumber,opening_bid,opening_ask,closing_bid,closing_ask,volume
		MarketData(u32,u32,FixedU128,FixedU128,FixedU128,FixedU128,FixedU128),
		// FIXME( Currently we iterate over all the trading pairs and emit events which is expensive)
		// TODO: Emit Market Data for only those markets which changed during the block.
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Error occured due to a Overflow during calculation
		CalculationOverflow,
		/// Order Failed to pass basic order checks
		BasicOrderChecksFailed,
		/// Same assets cannot be traded
		SameAssetIdsError,
		/// Zero Balance in both assets during registration
		InsufficientAssetBalance
	}
}

decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		/// Storage items related to DEX Starts here
		Books get(fn books): map hasher(blake2_128_concat) u32 => engine::OrderBook<T::AccountId,T::BlockNumber,T::AssetId>;

		BookId get(fn book_id): u32;
	}
}


// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		// TODO: Note for enabling feeless trades use dispatch::DispatchResultWithPostInfo
		// TODO: then in the Ok(()) replace it with Ok(Some(0).into()) to make it fee-less

		/// This is used to list a new trading pair in the DEX. The origin has to reserve the
		/// TokenListingFee + PairListingFee if the token is not already available in DEX else
		/// only the PairListingFee is reserved until the token is de-listed from the DEX.
		/// Origin will not have any interest. It will avoid abusing the DEX with invaluable tokens
		/// and trading pairs.
		/// trading pair notation: trading_asset/base_asset
		#[weight = 10000]
		pub fn register_new_orderbook(origin, trading_asset_id: u32, base_asset_id: u32) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Save the AssetIds check if it's valid and create the the orderbook for the given
		// TODO: pair


		// If assets ids are same then it's error
		if &trading_asset_id == &base_asset_id {
		Self::deposit_event(RawEvent::SameAssetIdsError(trading_asset_id, base_asset_id));
		return Err(<Error<T>>::SameAssetIdsError.into());
		}
		// The origin should have a non-zero balance in both assets
		let trading_asset_balance = pallet_generic_asset::Module::<T>::free_balance(&Self::u32_to_asset_id(trading_asset_id), &_trader);
		let base_asset_balance = pallet_generic_asset::Module::<T>::free_balance(&Self::u32_to_asset_id(base_asset_id), &_trader);
		if (TryInto::<u128>::try_into(trading_asset_balance).ok().unwrap()>0) && (TryInto::<u128>::try_into(base_asset_balance).ok().unwrap()>0){
		// The origin should reserve a certain amount of SpendingAssetCurrency for registering the pair

		if Self::reserve_balance_registration(&_trader){
		// Create the orderbook
		let trading_pair_id = Self::create_order_book(Self::u32_to_asset_id(trading_asset_id),Self::u32_to_asset_id(base_asset_id));
		Self::deposit_event(RawEvent::TradingPairCreated(trading_pair_id));
		return Ok(());
		}else{
		return Err(<Error<T>>::InsufficientAssetBalance.into());
		}
		}else{
		// If the balance of either one asset of trading pair is non zero, return error.
		Self::deposit_event(RawEvent::NoBalanceOfAssets(TryInto::<u128>::try_into(trading_asset_balance).ok().unwrap(),
		TryInto::<u128>::try_into(base_asset_balance).ok().unwrap()));
		return Err(<Error<T>>::InsufficientAssetBalance.into());
		}
		}

		/// This function can be used to submit limit orders
		/// Trading pair notation: trading_asset/base_asset ie (BTC/USDT)
		/// Price is BTC/USDT and Quantity is BTC
		#[weight = 10000]
		pub fn submit_limit_order(origin,
		  order_type: engine::OrderType,
		  order_id: sp_std::vec::Vec<u8>,
		  price: FixedU128,
		  quantity: FixedU128,
		  trading_pair: u32) -> dispatch::DispatchResult{
		let trader = ensure_signed(origin)?;

		match Self::basic_order_checks(&trader,trading_pair,price,quantity,order_type.clone(),order_id.clone()){

		Some(mut order_book) => {
		// TODO: Reserve the balance
		// TODO: Update the market data struct
		// TODO: Try to execute order else put it in the order book
		// TODO: Update the market data struct
		order_book = Self::execute_normal_order(order_book,order_type.clone(),order_id.clone(),price,quantity,trading_pair,&trader);
		<Books<T>>::insert(trading_pair,order_book); // Modifies the state to insert new order_book
		// let trade_amount = Self::calculate_trade_amount(price,quantity).ok_or(<Error<T>>::CalculationOverflow)?;
		// if let Some(trade_amount_converted) = Self::convert_fixed_u128_to_balance(trade_amount){
		// // Emit Event
		// Self::deposit_event(RawEvent::TradeAmount(trade_amount_converted, trade_amount, trader));
		// }else{
		// return Err(<Error<T>>::BasicOrderChecksFailed.into());
		// }
		         },
		None => {
		return Err(<Error<T>>::BasicOrderChecksFailed.into());
		        }
		    }
		Ok(())
		}

		// This function can be used to submit market orders
		#[weight = 10000]
		pub fn submit_market_order(origin,
		  order_type: engine::OrderType,
		  order_id: sp_std::vec::Vec<u8>,
		  price: FixedU128,
		  quantity: FixedU128 ) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given market order.
		Ok(())
		}

		// This function can be used to submit advanced orders
		#[weight = 10000]
		pub fn submit_advanced_order(origin,
		  order_type: engine::OrderType,
		  order_id: sp_std::vec::Vec<u8>,
		  price: FixedU128,
		  quantity: FixedU128 ) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given advanced order.
		Ok(())
		}

		// This function can be used to cancel orders
		#[weight = 10000]
		pub fn cancel_order(origin, order_id: sp_std::vec::Vec<u8>) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the cancel order logic for the given orderID.
		Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
    // fn encode_and_update_nonce() -> Vec<u8> {
    //     let nonce = Nonce::get();
    //     Nonce::put(nonce.wrapping_add(1));
    //     nonce.encode()
    // }

    fn create_order_book(trading_asset_id: T::AssetId, base_asset_id: T::AssetId) -> u32 {
        let current_id = Self::book_id();
        let current_block_num = <frame_system::Module<T>>::current_block_number();
        let order_book: engine::OrderBook<T::AccountId, T::BlockNumber, T::AssetId> = engine::OrderBook {
            id: current_id,
            trading_asset: trading_asset_id,
            base_asset: base_asset_id,
            nonce: 0,
            orders: btree_map::BTreeMap::new(),
            advanced_bid_orders: binary_heap::BinaryHeap::new(),
            advanced_ask_orders: binary_heap::BinaryHeap::new_min(),
            bids: binary_heap::BinaryHeap::new(),
            asks: binary_heap::BinaryHeap::new_min(),
            market_data: sp_std::vec![engine::MarketData{
                current_block:  current_block_num,
                opening_bid: FixedU128::from(0),
                opening_ask: FixedU128::from(0),
                closing_bid: FixedU128::from(0),
                closing_ask: FixedU128::from(0),
                volume: FixedU128::from(0)
            }],
            enabled: true,
        };
        let tradingpair = order_book.id.clone();
        BookId::put(current_id + 1);
        Books::<T>::insert(order_book.id as u32, order_book);
        return tradingpair;
    }

    fn reserve_balance_registration(origin: &<T as frame_system::Trait>::AccountId) -> bool {
        pallet_generic_asset::Module::<T>::reserve(
            &pallet_generic_asset::SpendingAssetIdProvider::<T>::asset_id(),
            origin, 1000000.into()).is_ok()   // TODO: Fix a new amount using Configuration Trait
    }
    fn u32_to_asset_id(input: u32) -> T::AssetId {
        input.into()
    }

    /// Call this function to calculate trade amount based on given price and quantity
    fn calculate_trade_amount(price: FixedU128, quantity: FixedU128) -> Option<FixedU128> {
        price.checked_mul(&quantity)
    }

    /// Checks trading pair
    /// Checks balance
    /// Checks order id
    /// Checks order_type for Valid order type
    /// Checks if price & quantity is Zero
    /// Provides Orderbook for modification, reducing calls to storage
    /// Note: Price is in (base_asset/trading_asset) and Quantity is in trading_asset
    /// Trading pair notation: trading_asset/base_asset ie (BTC/USDT)
    /// Price is BTC/USDT and Quantity is BTC
    fn basic_order_checks(origin: &<T as frame_system::Trait>::AccountId, trading_pair: u32,
                          price: FixedU128, quantity: FixedU128, order_type: engine::OrderType,
                          order_id: sp_std::vec::Vec<u8>) -> Option<engine::OrderBook<T::AccountId, T::BlockNumber, T::AssetId>> {
        if price <= FixedU128::from(0) && quantity <= FixedU128::from(0) {
            Self::deposit_event(RawEvent::PriceOrQuanitityIsZero);
            return None;
        }
        if order_type == engine::OrderType::None {
            Self::deposit_event(RawEvent::OrderTypeIsNone);
            return None;
        }
        if !(<Books<T>>::contains_key(trading_pair)) {
            Self::deposit_event(RawEvent::TradingPairNotFound(trading_pair));
            return None;
        }

        let order_book: engine::OrderBook<T::AccountId, T::BlockNumber, T::AssetId> = <Books<T>>::get(trading_pair);

        let trading_asset_id = order_book.get_trading_asset();
        let base_asset_id = order_book.get_base_asset();
        let orders = order_book.get_orders();

        match order_type {
            engine::OrderType::AskLimit | engine::OrderType::AskMarket => {
                // Check if that much quantity is available
                let trading_balance = pallet_generic_asset::Module::<T>::free_balance(&trading_asset_id, &origin);
                if let Some(trading_balance_converted) = Self::convert_balance_to_fixed_u128(trading_balance) {
                    if Self::has_balance_for_trading(orders, trading_balance_converted, quantity, order_id) {
                        Some(order_book)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            engine::OrderType::BidLimit | engine::OrderType::BidMarket => {
                //  Check if price*quantity is available in the base_asset.
                let base_balance = pallet_generic_asset::Module::<T>::free_balance(&base_asset_id, &origin);
                if let Some(base_balance_converted) = Self::convert_balance_to_fixed_u128(base_balance) {
                    if let Some(computed_trade_amount) = (&price).checked_mul(&quantity) {
                        if Self::has_balance_for_trading(orders, base_balance_converted, computed_trade_amount, order_id) {
                            Some(order_book)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => {
                None
            }
        }
    }

    /// Checks if the given order id exists in the given orderbook
    fn check_order_id(orders: &btree_map::BTreeMap<Vec<u8>, engine::Order<T::AccountId, T::BlockNumber>>
                      , order_id: sp_std::vec::Vec<u8>) -> bool {
        if orders.contains_key(&order_id) {
            Self::deposit_event(RawEvent::DuplicateOrderId(order_id));
            false
        } else {
            true
        }
    }

    /// Checks if the balance is enough to execute given trade and returns the orderbook
    fn has_balance_for_trading(orders: &btree_map::BTreeMap<Vec<u8>, engine::Order<T::AccountId, T::BlockNumber>>,
                               balance_to_check: FixedU128,
                               computed_amount: FixedU128,
                               order_id: sp_std::vec::Vec<u8>)
                               -> bool {
        return if balance_to_check >= computed_amount {
            if Self::check_order_id(&orders, order_id) {
                true
            } else {
                false
            }
        } else {
            Self::deposit_event(RawEvent::InsufficientAssetBalance(balance_to_check));
            false
        };
    }

    /// Converts Balance to FixedU128 representation
    pub fn convert_balance_to_fixed_u128(x: T::Balance) -> Option<FixedU128> {
        if let Some(y) = TryInto::<u128>::try_into(x).ok() {
            FixedU128::from(y).checked_div(&FixedU128::from(1_000_000_000_000))
        } else {
            None
        }
    }

    /// Converts FixedU128 to Balance representation
    pub fn convert_fixed_u128_to_balance(x: FixedU128) -> Option<T::Balance> {
        if let Some(balance_in_fixed_u128) = x.checked_div(&FixedU128::from(1000000)) {
            let balance_in_u128 = balance_in_fixed_u128.into_inner();
            Some(UniqueSaturatedFrom::<u128>::unique_saturated_from(balance_in_u128))
        } else {
            None
        }
    }

    fn execute_normal_order(mut order_book: engine::OrderBook<T::AccountId, T::BlockNumber, T::AssetId>,
                            order_type: engine::OrderType,
                            order_id: sp_std::vec::Vec<u8>,
                            price: FixedU128,
                            quantity: FixedU128,
                            trading_pair: u32,
                            trader: &<T as frame_system::Trait>::AccountId)
                            -> OrderBook<T::AccountId, T::BlockNumber, T::AssetId> {
        // TODO: There might be lot of problems with borrow and reference management
        // TODO: A lot of checking is required
        let mut current_order: engine::Order<T::AccountId, T::BlockNumber> = engine::Order {
            id: order_id.clone(),
            order_type: order_type.clone(),
            price: price.clone(),
            quantity: quantity.clone(),
            market_maker: false,
            origin: trader.clone(),
            expiry: T::BlockNumber::default(), // We ignore this expiry as it is only used for advanced orders
        };
        let trading_asset_id = *order_book.get_trading_asset();
        let base_asset_id = *order_book.get_base_asset();
        match order_type {
            // Buy Limit Order
            engine::OrderType::BidLimit => {
                let asks = order_book.get_asks();
                loop {
                    if let Some(counter_price_level_peek) = asks.peek() {
                        if counter_price_level_peek.get_price_level() <= &price {
                            if let Some(mut counter_price_level) = asks.pop() {
                                // There are orders and counter_price_level matches asked_price_level
                                let orders = counter_price_level.get_orders_mut();
                                let mut matched = false;
                                for index in 0..orders.len() {
                                    if let Some(mut counter_order) = orders.remove(index) {
                                        let counter_quantity = counter_order.get_quantity();
                                        if counter_quantity > current_order.get_quantity() {
                                            // partially execute counter order
                                            counter_order = Self::partially_execute_order(counter_order,
                                                                                          &current_order,
                                                                                          trading_asset_id.clone(),
                                                                                          base_asset_id.clone());
                                            // push front the remaining counter order
                                            orders.push_front(counter_order);
                                            matched = true;
                                            break;
                                        } else if counter_quantity == current_order.get_quantity() {
                                            // fully execute current order
                                            // fully execute counter order
                                            Self::fully_execute_order(counter_order,
                                                                      &current_order,
                                                                      trading_asset_id.clone(),
                                                                      base_asset_id.clone());
                                            // Remove both orders
                                            matched = true;
                                            break;
                                        } else {
                                            // partially execute current order
                                            current_order = Self::partially_execute_order(current_order,
                                                                                          &counter_order,
                                                                                          trading_asset_id.clone(),
                                                                                          base_asset_id.clone());
                                            // pop another order from queue or insert new bid in bids
                                        }
                                    }
                                }
                                if matched {
                                    // current order executed completely
                                    // save the state and exit
                                    // If there are orders left in this price level put it back in the BinaryHeap
                                    if counter_price_level.get_orders().len() != 0 {
                                        asks.push(counter_price_level);
                                    }
                                    break;
                                }
                            } else {
                                // It will not occur as peek will be None if price level is not there
                            }
                        } else {
                            // There are orders but not at asked price_level so add this order to bids
                            // current_order is market maker so no fees
                            let bids = order_book.get_bids();
                            let mut price_level_match = false;
                            current_order = match bids.peek() {
                                Some(current_price_level) => {
                                    if current_price_level.get_price_level() != current_order.get_price() {
                                        // current_order price and price_level doesn't match
                                        // Create one and put it in
                                        let mut new_price_level = engine::PriceLevel {
                                            price_level: *current_order.get_price(),
                                            queue: VecDeque::new(),
                                        };
                                        new_price_level.get_orders_mut().push_back(current_order.clone());
                                        bids.push(new_price_level);
                                        current_order
                                    } else {
                                        // Price and price_level matches
                                        // Add it to queue of this price_level
                                        price_level_match = true;
                                        current_order
                                    }
                                }
                                None => {
                                    // There are not price levels available
                                    // Create one and put it in
                                    let mut new_price_level = engine::PriceLevel {
                                        price_level: *current_order.get_price(),
                                        queue: VecDeque::new(),
                                    };
                                    new_price_level.get_orders_mut().push_back(current_order.clone());
                                    bids.push(new_price_level);
                                    current_order
                                }
                            };
                            if price_level_match {
                                if let Some(mut current_price_level) = bids.peek_mut() {
                                    current_price_level.get_orders_mut().push_back(current_order);
                                } else {}
                            }
                            // if let Some(mut current_price_level) = bids.peek_mut() {
                            //
                            // } else {
                            //
                            // }
                            // order_book.bids = *bids;
                            break;
                        }
                    } else {
                        // There are no orders in the heap so add this order to bids
                        // current_order is market maker so no fees
                        // There are orders but not at asked price_level so add this order to bids
                        // current_order is market maker so no fees
                        let bids = order_book.get_bids();
                        let mut price_level_match = false;
                        current_order = match bids.peek() {
                            Some(current_price_level) => {
                                if current_price_level.get_price_level() != current_order.get_price() {
                                    // current_order price and price_level doesn't match
                                    // Create one and put it in
                                    let mut new_price_level = engine::PriceLevel {
                                        price_level: *current_order.get_price(),
                                        queue: VecDeque::new(),
                                    };
                                    new_price_level.get_orders_mut().push_back(current_order.clone());
                                    bids.push(new_price_level);
                                    current_order
                                } else {
                                    // Price and price_level matches
                                    // Add it to queue of this price_level
                                    price_level_match = true;
                                    current_order
                                }
                            }
                            None => {
                                // There are not price levels available
                                // Create one and put it in
                                let mut new_price_level = engine::PriceLevel {
                                    price_level: *current_order.get_price(),
                                    queue: VecDeque::new(),
                                };
                                new_price_level.get_orders_mut().push_back(current_order.clone());
                                bids.push(new_price_level);
                                current_order
                            }
                        };
                        if price_level_match {
                            if let Some(mut current_price_level) = bids.peek_mut() {
                                current_price_level.get_orders_mut().push_back(current_order);
                            } else {
                                // Throw an Error here
                            }
                        }
                        break;
                    }
                }

                order_book
            }
            // Sell Limit Order
            engine::OrderType::AskLimit => {
                order_book
            }
            // Buy Market Order
            engine::OrderType::BidMarket => {
                order_book
            }
            // Sell Market Order
            engine::OrderType::AskMarket => {
                order_book
            }
            // TODO:  Ignores other cases maybe should we generate an event for it?
            _ => {
                order_book
            }
        }
    }

    // For the sake of understanding
    fn partially_execute_order(executing_order: engine::Order<T::AccountId, T::BlockNumber>,
                               trigger_order: &engine::Order<T::AccountId, T::BlockNumber>,
                               trading_asset_id: T::AssetId,
                               base_asset_id: T::AssetId) -> Order<T::AccountId, T::BlockNumber> {
        return Self::fully_execute_order(executing_order, trigger_order, trading_asset_id, base_asset_id);
    }

    fn fully_execute_order(mut executing_order: engine::Order<T::AccountId, T::BlockNumber>,
                           trigger_order: &engine::Order<T::AccountId, T::BlockNumber>,
                           trading_asset_id: T::AssetId,
                           base_asset_id: T::AssetId) -> Order<T::AccountId, T::BlockNumber> {
        return match executing_order.get_order_type() {
            engine::OrderType::BidLimit | engine::OrderType::BidMarket => {
                // TODO: Remove the unwraps it can cause a panic
                let trade_amount: T::Balance = Self::convert_fixed_u128_to_balance(
                    executing_order.get_price().checked_mul(trigger_order.get_quantity()).unwrap()).unwrap();
                let trigger_quantity: T::Balance = Self::convert_fixed_u128_to_balance(*trigger_order.get_quantity()).unwrap();
                // un-reserve price*quantity of base_asset from executing_order's origin
                pallet_generic_asset::Module::<T>::unreserve(&base_asset_id,
                                                             executing_order.get_origin(),
                                                             trade_amount);
                // un-reserve quantity of trading_asset from trigger_order's origin
                pallet_generic_asset::Module::<T>::unreserve(&trading_asset_id,
                                                             trigger_order.get_origin(),
                                                             trigger_quantity);
                // TODO: Check the results for OK()
                // Transfer price*quantity of base_asset from executing_order's origin to trigger_order's origin
                let _result = pallet_generic_asset::Module::<T>::make_transfer(&base_asset_id,
                                                                               executing_order.get_origin(),
                                                                               trigger_order.get_origin(),
                                                                               trade_amount);
                // Transfer quantity of trading_asset from trigger_order's origin to executing_order's origin
                let _result = pallet_generic_asset::Module::<T>::make_transfer(&trading_asset_id,
                                                                               trigger_order.get_origin(),
                                                                               executing_order.get_origin(),
                                                                               trigger_quantity);

                // TODO: Remove the unwraps it can cause a panic
                executing_order.set_quantity(executing_order.get_quantity().checked_sub(trigger_order.get_quantity()).unwrap());
                // TODO: Deposit events for partial fill of executing_order
                // TODO: Deposit events for complete fill of trigger_order
                executing_order
            }
            engine::OrderType::AskLimit | engine::OrderType::AskMarket => {
                // TODO: Remove the unwraps it can cause a panic
                let trade_amount: T::Balance = Self::convert_fixed_u128_to_balance(
                    executing_order.get_price().checked_mul(trigger_order.get_quantity()).unwrap()).unwrap();
                let trigger_quantity: T::Balance = Self::convert_fixed_u128_to_balance(*trigger_order.get_quantity()).unwrap();
                // un-reserve quantity of trading_asset from executing_order's origin
                pallet_generic_asset::Module::<T>::unreserve(&trading_asset_id,
                                                             executing_order.get_origin(),
                                                             trigger_quantity);
                // un-reserve price*quantity of base_asset from trigger_order's origin
                pallet_generic_asset::Module::<T>::unreserve(&base_asset_id,
                                                             trigger_order.get_origin(),
                                                             trade_amount);
                // TODO: Check the results for OK()
                // Transfer quantity of trading_asset from executing_order's origin to trigger_order's origin
                let _result = pallet_generic_asset::Module::<T>::make_transfer(&trading_asset_id,
                                                                               executing_order.get_origin(),
                                                                               trigger_order.get_origin(),
                                                                               trigger_quantity);
                // Transfer price*quantity of base_asset from trigger_order's origin to executing_order's origin
                let _result = pallet_generic_asset::Module::<T>::make_transfer(&base_asset_id,
                                                                               trigger_order.get_origin(),
                                                                               executing_order.get_origin(),
                                                                               trade_amount);

                // TODO: Remove the unwraps it can cause a panic
                executing_order.set_quantity(executing_order.get_quantity().checked_sub(trigger_order.get_quantity()).unwrap());
                // TODO: Deposit events for partial fill of executing_order
                // TODO: Deposit events for complete fill of trigger_order
                executing_order
            }
            _ => {
                // Ignore other patterns
                executing_order
            }
        };
    }
}