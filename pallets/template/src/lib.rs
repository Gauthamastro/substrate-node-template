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
		InsufficientAssetBalance(U32F32),
		/// Order contains a duplicate orderId of another active order
		DuplicateOrderId(Vec<u8>),
		/// Order type of Order is None
		OrderTypeIsNone,
		/// Invalid TradingPair Id
		TradingPairNotFound(u32),
		/// Same Assets cannot be traded
		SameAssetIdsError(u32,u32),
		/// Zero Balances in either one or both Assets
		NoBalanceOfAssets(u128,u128),
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

		// This is used to list a new trading pair in the DEX. The origin has to reserve the
		// TokenListingFee + PairListingFee if the token is not already available in DEX else
		// only the PairListingFee is reserved until the token is de-listed from the DEX.
		// Origin will not have any interest. It will avoid abusing the DEX with invaluable tokens
		// and trading pairs.
		#[weight = 10000]
		pub fn register_new_orderbook(origin, trading_asset_id: u32, quote_asset_id: u32) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Save the AssetIds check if it's valid and create the the orderbook for the given
		// TODO: pair
		if &trading_asset_id == &quote_asset_id {
		Self::deposit_event(RawEvent::SameAssetIdsError(trading_asset_id, quote_asset_id));
		return Err(<Error<T>>::SameAssetIdsError.into());
		}
		let trading_asset_balance = pallet_generic_asset::Module::<T>::free_balance(&Self::u32_to_asset_id(trading_asset_id), &_trader);
		let quote_asset_balance = pallet_generic_asset::Module::<T>::free_balance(&Self::u32_to_asset_id(quote_asset_id), &_trader);
		if (TryInto::<u128>::try_into(trading_asset_balance).ok().unwrap()>0) && (TryInto::<u128>::try_into(quote_asset_balance).ok().unwrap()>0){

		}else{
		Self::deposit_event(RawEvent::NoBalanceOfAssets(TryInto::<u128>::try_into(trading_asset_balance).ok().unwrap(),
		TryInto::<u128>::try_into(quote_asset_balance).ok().unwrap()));
		return Err(<Error<T>>::InsufficientAssetBalance.into());
		}

		Ok(())
		}

		// This function can be used to submit limit orders
		#[weight = 10000]
		pub fn submit_limit_order(origin,
		  order_type: engine::OrderType,
		  order_id: sp_std::vec::Vec<u8>,
		  price: U32F32,
		  quantity: U32F32,
		  trading_pair: u32) -> dispatch::DispatchResult{
		let trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given limit order.
		match Self::basic_order_checks(&trader,trading_pair,price,quantity,order_type,order_id){

		Some(_order_book) => {
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
		  order_id: T::Hash,
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
		  order_id: T::Hash,
		  price: U32F32,
		  quantity: U32F32 ) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given advanced order.
		Ok(())
		}

		// This function can be used to cancel orders
		#[weight = 10000]
		pub fn cancel_order(origin, order_id: T::Hash) -> dispatch::DispatchResult{
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
    fn u32_to_asset_id(input: u32) -> T::AssetId{
        input.into()
    }

    /// Call this function to calculate trade amount based on given price and quantity
    fn calculate_trade_amount(price: U32F32, quantity: U32F32) -> Option<U32F32> {
        price.checked_mul(quantity)
    }

    /// Check trading pair
    /// Check balance
    /// Check order id
    /// Check order_type for Valid order type
    /// TODO: Check if price & quantity is Zero
    /// TODO: Get reference to orderbook from the calling function to avoid accessing storage again
    fn basic_order_checks(origin: &<T as frame_system::Trait>::AccountId, trading_pair: u32,
                          price: U32F32, quantity: U32F32, order_type: engine::OrderType,
                          order_id: sp_std::vec::Vec<u8>) -> Option<engine::OrderBook<T::AccountId, T::BlockNumber, T::AssetId>> {
        if !(<Books<T>>::contains_key(trading_pair)) {
            Self::deposit_event(RawEvent::TradingPairNotFound(trading_pair));
            return None;
        } else if order_type == engine::OrderType::None {
            Self::deposit_event(RawEvent::OrderTypeIsNone);
            return None;
        }
        let order_book: engine::OrderBook<T::AccountId, T::BlockNumber, T::AssetId> = <Books<T>>::get(trading_pair);

        let trading_asset_id = order_book.get_trading_asset();
        // let quote_asset_id = orderBook.get_quote_asset();
        let temp = pallet_generic_asset::Module::<T>::free_balance(&trading_asset_id, &origin);
        let temp_converted = TryInto::<u128>::try_into(temp).ok().unwrap();
        return if U32F32::from_num(temp_converted) >= (&price).checked_mul(quantity).unwrap() {
            if order_book.get_orders().contains_key(&order_id) {
                Self::deposit_event(RawEvent::DuplicateOrderId(order_id));
                None
            } else {
                Some(order_book)
            }
        } else {
            Self::deposit_event(RawEvent::InsufficientAssetBalance((&price).checked_mul(quantity).unwrap()));
            None
        };
    }
}