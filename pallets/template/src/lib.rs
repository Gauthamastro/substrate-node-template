#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;
// use binary_heap_plus::{BinaryHeap, MaxComparator};
use sp_std::str;
use sp_std::vec::Vec;
use substrate_fixed::types::U32F32;

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
		SomethingStored(u32, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;

		/// Storage items related to DEX Starts here
		Books get(fn books): map hasher(blake2_128_concat) u32 => engine::OrderBook<T::AccountId>;

		// OrdersByAccountId get(fn orders_by_accountId):
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

		Ok(())
		}

		// This function can be used to submit limit orders
		#[weight = 10000]
		pub fn submit_limit_order(origin,
		  order_type: engine::OrderType,
		  price: u128,
		  quantity: u128 ) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given limit order.

		Ok(())
		}

		// This function can be used to submit market orders
		#[weight = 10000]
		pub fn submit_market_order(origin,
		  order_type: engine::OrderType,
		  price: u128,
		  quantity: u128 ) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given market order.
		Ok(())
		}

		// This function can be used to submit advanced orders
		#[weight = 10000]
		pub fn submit_advanced_order(origin,
		  order_type: engine::OrderType,
		  price: u128,
		  quantity: u128 ) -> dispatch::DispatchResult{
		let _trader = ensure_signed(origin)?;
		// TODO: Do the order logic for the given advanced order.
		Ok(())
		}

		// This function can be used to cancel orders
		#[weight = 10000]
		pub fn cancel_order(origin, order_id: Vec<u8>) -> dispatch::DispatchResult{
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
}