#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;
use sp_std::str;
use substrate_fixed::types::{U32F32};
use sp_std::convert::TryInto;
use sp_std::vec::Vec;
use pallet_generic_asset::AssetIdProvider;

mod binary_heap;
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
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		TradeAmount(U32F32, AccountId),
		/// Not enough asset free balance for placing the trade
		InsufficientAssetBalance(u128),
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
		/// Contains market state about current block.
		/// Order: tradingPair,blockNumber,opening_bid,opening_ask,closing_bid,closing_ask,volume
		MarketData(u32,u32,U32F32,U32F32,U32F32,U32F32,U32F32),
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
		  price: U32F32,
		  quantity: U32F32,
		  trading_pair: u32) -> dispatch::DispatchResult{
		let trader = ensure_signed(origin)?;

		match Self::basic_order_checks(&trader,trading_pair,price,quantity,order_type,order_id){

		Some(order_book) => {
		// TODO: Reserve the balance
		// TODO: Update the market data struct
		// TODO: Try to execute order else put it in the order book
		// TODO: Update the market data struct
		// Refer the fixed point to floating point converter for more information. The given function works!!
		let trade_amount = Self::calculate_trade_amount(price,quantity).ok_or(<Error<T>>::CalculationOverflow)?;
		// Emit Event
		Self::deposit_event(RawEvent::TradeAmount(trade_amount, trader));
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
		  price: U32F32,
		  quantity: U32F32 ) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given market order.
		Ok(())
		}

		// This function can be used to submit advanced orders
		#[weight = 10000]
		pub fn submit_advanced_order(origin,
		  order_type: engine::OrderType,
		  order_id: sp_std::vec::Vec<u8>,
		  price: U32F32,
		  quantity: U32F32 ) -> dispatch::DispatchResult{
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

use sp_std::collections::btree_map;
use frame_support::sp_runtime::offchain::storage_lock::BlockNumberProvider;
use frame_support::traits::IsType;

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
                opening_bid: U32F32::from_num(0),
                opening_ask: U32F32::from_num(0),
                closing_bid: U32F32::from_num(0),
                closing_ask: U32F32::from_num(0),
                volume: U32F32::from_num(0)
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
    fn calculate_trade_amount(price: U32F32, quantity: U32F32) -> Option<U32F32> {
        price.checked_mul(quantity)
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
                          price: U32F32, quantity: U32F32, order_type: engine::OrderType,
                          order_id: sp_std::vec::Vec<u8>) -> Option<engine::OrderBook<T::AccountId, T::BlockNumber, T::AssetId>> {
        if price <= 0 && quantity <= 0 {
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
                // TODO: unwrap() has a chance to panic. Remove it!
                if let Some(trading_balance_converted ) = TryInto::<u128>::try_into(trading_balance).ok(){
                    if Self::has_balance_for_trading(orders.into_ref(), trading_balance_converted, quantity, order_id) {
                        Some(order_book)
                    } else {
                        None
                    }
                }else{
                    None
                }
            }
            engine::OrderType::BidLimit | engine::OrderType::BidMarket => {
                //  Check if price*quantity is available in the base_asset.
                let base_balance = pallet_generic_asset::Module::<T>::free_balance(&base_asset_id, &origin);
                // TODO: unwrap() has a chance to panic. Remove it!
                if let Some(base_balance_converted) = TryInto::<u128>::try_into(base_balance).ok(){
                    let computed_trade_amount = (&price).checked_mul(quantity).unwrap();
                    if Self::has_balance_for_trading(orders.into_ref(), base_balance_converted, computed_trade_amount, order_id) {
                        Some(order_book)
                    } else {
                        None
                    }
                }else{
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
                               balance_to_check: u128,
                               computed_amount: U32F32,
                               order_id: sp_std::vec::Vec<u8>)
                               -> bool {
        return if U32F32::from_num(balance_to_check) >= computed_amount {
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
}