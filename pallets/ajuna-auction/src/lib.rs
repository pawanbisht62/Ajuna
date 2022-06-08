// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	sp_runtime::traits::{
		AtLeast32BitUnsigned, Bounded, CheckedAdd, CheckedSub,
		One, StaticLookup, Zero,
	},
	traits::{
		Currency, LockIdentifier, LockableCurrency, WithdrawReasons, ExistenceRequirement,
	},
};
use sp_runtime::Permill;
use sp_std::result;

use sp_std::prelude::*;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

use weights::WeightInfo;
pub mod weights;

pub use traits::*;
pub mod traits;

//use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Identifier for the currency lock on accounts
const AUCTION_LOCK_ID: LockIdentifier = *b"_auction";
/// Set in percent how much next bid has to be raised
const BID_STEP_PERC: u32 = 10;
/// Increase endtime to avoid sniping
const BID_ADD_BLOCKS: u32 = 10;
/// Minimal auction duration
const MIN_AUCTION_DUR: u32 = 10;

/// Define type aliases for better readability
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type NftClassIdOf<T> = pallet_ajuna_nft::ClassIdOf<T>;
pub type NftTokenIdOf<T> = pallet_ajuna_nft::TokenIdOf<T>;
pub type AuctionInfoOf<T> = AuctionInfo<
	<T as frame_system::Config>::AccountId,
	BalanceOf<T>,
	<T as frame_system::Config>::BlockNumber,
	NftClassIdOf<T>,
	NftTokenIdOf<T>,
>;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*, sp_runtime::traits::TrailingZeroInput,
	};
	use frame_system::pallet_prelude::*;

	// important to use outside structs and consts
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_ajuna_nft::Config {

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The balance type for bidding
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize;

		/// The auction ID type
		type AuctionId: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ Bounded
			+ CheckedAdd;

		/// Single type currency (TODO multiple currencies)
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

		// This type is needed to convert from Currency to Balance
		type CurrencyBalance: From<Self::Balance>
			+ Into<<Self::Currency as Currency<<Self as frame_system::Config>::AccountId>>::Balance>;

		/// Weights
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn auctions)]
	/// Stores on-going and future auctions. Closed auction are removed.
	// TODO: use single Auction storage using double map (auctionId, type)
	pub type Auctions<T: Config> = StorageMap<_, Twox64Concat, T::AuctionId, AuctionInfoOf<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auctions_index)]
	/// Track the next auction ID.
	pub type NextAuctionId<T: Config> = StorageValue<_, T::AuctionId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auction_end_time)]
	/// Index auctions by end time.
	pub type AuctionEndTime<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::BlockNumber, Twox64Concat, T::AuctionId, (), OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn auction_owner_by_id)]
	/// Auction owner by ID
	pub type AuctionOwnerById<T: Config> = StorageMap<_, Twox64Concat, T::AuctionId, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn founder_key)]
	pub type FounderKey<T: Config> = StorageValue<_, T::AccountId>;

	// Used for generating a zeroed account id, copy pasted for now
	struct DefaultAccountIdGenerator<T: Config>(pub T::AccountId);

	impl<T: Config> Default for DefaultAccountIdGenerator<T> {
		fn default() -> Self {
			// Stolen from https://github.com/paritytech/substrate/commit/f57c6447af83a1706041d462ca290b4f2a1bac4f#diff-68096a50d12854e07693a4828590517bb81fea37a9253640278ecdc5b93b6992R860
			let zero_account_id = T::AccountId::decode(&mut TrailingZeroInput::zeroes())
				.expect("infinite length input; no invalid inputs for type; qed");

			DefaultAccountIdGenerator(zero_account_id)
		}
	}

	// The genesis config type.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub founder_key: T::AccountId,
	}

	// The default value for the genesis config type.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { founder_key: DefaultAccountIdGenerator::<T>::default().0 }
		}
	}

	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<FounderKey<T>>::put(&self.founder_key);
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Auction created
		AuctionCreated(T::AccountId, T::AuctionId),
		/// A bid is placed
		Bid(T::AuctionId, T::AccountId, BalanceOf<T>),
		/// Auction ended
		AuctionConcluded(T::AuctionId),
		/// Auction removed
		AuctionRemoved(T::AuctionId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AuctionNotExist,
		AuctionNotStarted,
		AuctionAlreadyStarted,
		BidNotAccepted,
		InvalidBidPrice,
		NoAvailableAuctionId,
		AuctionStartTimeAlreadyPassed,
		NonExistingAuctionType,
		InvalidTimeConfiguration,
		NotATokenOwner,
		AuctionAlreadyConcluded,
		BidOverflow,
		BidOnOwnAuction,
		TimeUnderflow,
		TokenLocked,
		EmptyAuctionName,
	}

	// Pallet implements [`Hooks`] trait to define some logic to execute in some context.
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// `on_initialize` is executed at the beginning of the block before any extrinsic are
		// dispatched.
		//
		// This function must return the weight consumed by `on_initialize` and `on_finalize`.
		fn on_initialize(_: T::BlockNumber) -> Weight {
			0
		}

		// `on_finalize` is executed at the end of block after all extrinsic are dispatched.
		fn on_finalize(now: BlockNumberFor<T>) {
			Self::conclude_auction(now);
		}

		// A runtime code run after every block and have access to extended set of APIs.
		//
		// For instance you can generate extrinsics for the upcoming produced block.
		fn offchain_worker(_n: T::BlockNumber) {
			// We don't do anything here.
			// but we could dispatch extrinsic (transaction/unsigned/inherent) using
			// sp_io::submit_extrinsic.
			// To see example on offchain worker, please refer to example-offchain-worker pallet
			// accompanied in this repository.
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(<T as Config>::WeightInfo::create_auction())]
		pub fn create_auction(origin: OriginFor<T>, auction_info: AuctionInfoOf<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let new_auction_id = Self::new_auction(auction_info)?;
			Self::deposit_event(Event::AuctionCreated(sender, new_auction_id));
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::bid_value())]
		pub fn bid_value(origin: OriginFor<T>, id: T::AuctionId, value: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			Self::bid(sender.clone(), id, value)?;
			Self::deposit_event(Event::Bid(id, sender, value));
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::delete_auction())]
		pub fn delete_auction(origin: OriginFor<T>, id: T::AuctionId) -> DispatchResultWithPostInfo {
			let _sender = ensure_signed(origin)?;

			Self::remove_auction(id)?;
			Self::deposit_event(Event::AuctionRemoved(id));
			Ok(().into())
		}

	}
}

impl<T: Config> Pallet<T> {
	fn conclude_auction(now: T::BlockNumber) {
		for (auction_id, _) in <AuctionEndTime<T>>::drain_prefix(&now) {
			if let Some(auction) = Self::auctions(auction_id) {
				pallet_ajuna_nft::Pallet::<T>::toggle_lock(&auction.owner, auction.token_id).unwrap_or_default();
				// there is a bid so let's determine a winner and transfer tokens
				if let Some(ref winner) = auction.last_bid {
					let dest = T::Lookup::unlookup(winner.0.clone());
					let source = T::Origin::from(frame_system::RawOrigin::Signed(auction.owner.clone()));
					pallet_ajuna_nft::Pallet::<T>::transfer(source, dest, auction.token_id).unwrap_or_default();
					T::Currency::remove_lock(AUCTION_LOCK_ID, &winner.0);
					<T::Currency as Currency<T::AccountId>>::transfer(
						&winner.0,
						&auction.owner,
						winner.1,
						ExistenceRequirement::KeepAlive,
					)
					.unwrap_or_default();
				}
			}
		}
	}

	fn check_new_auction(info: &AuctionInfoOf<T>) -> DispatchResult {
		let current_block_number = frame_system::Pallet::<T>::block_number();
		ensure!(
			info.start >= current_block_number,
			Error::<T>::AuctionStartTimeAlreadyPassed
		);
		ensure!(
			info.start >= Zero::zero() && info.end > Zero::zero() && info.end > info.start + MIN_AUCTION_DUR.into(),
			Error::<T>::InvalidTimeConfiguration
		);
		ensure!(!info.name.is_empty(), Error::<T>::EmptyAuctionName);
		let is_owner = pallet_ajuna_nft::Pallet::<T>::is_owner(&info.owner, info.token_id);
		ensure!(is_owner, Error::<T>::NotATokenOwner);
		let nft_locked = pallet_ajuna_nft::Pallet::<T>::is_locked(info.token_id)?;
		ensure!(!nft_locked, Error::<T>::TokenLocked);
		Ok(())
	}
}

impl<T: Config> Auction<T::AccountId, T::BlockNumber, NftClassIdOf<T>, NftTokenIdOf<T>> for Pallet<T> {
	type AuctionId = T::AuctionId;
	type Balance = BalanceOf<T>;
	type AccountId = T::AccountId;

	fn new_auction(info: AuctionInfoOf<T>) -> result::Result<Self::AuctionId, DispatchError> {
		// Basic checks before an auction is created
		Self::check_new_auction(&info)?;
		let auction_id = <NextAuctionId<T>>::try_mutate(|next_id| -> result::Result<Self::AuctionId, DispatchError> {
			let current_id = *next_id;
			*next_id = next_id
				.checked_add(&One::one())
				.ok_or(Error::<T>::NoAvailableAuctionId)?;
			Ok(current_id)
		})?;

		<Auctions<T>>::insert(auction_id, info.clone());
		<AuctionOwnerById<T>>::insert(auction_id, &info.owner);
		<AuctionEndTime<T>>::insert(info.end, auction_id, ());
		pallet_ajuna_nft::Pallet::<T>::toggle_lock(&info.owner, info.token_id).unwrap_or_default();

		Ok(auction_id)
	}

	fn update_auction(id: Self::AuctionId, info: AuctionInfoOf<T>) -> DispatchResult {
		<Auctions<T>>::try_mutate(id, |auction| -> DispatchResult {
			ensure!(auction.is_some(), Error::<T>::AuctionNotExist);
			*auction = Some(info);
			Ok(())
		})
	}

	fn remove_auction(id: Self::AuctionId) -> DispatchResult {
		let auction = <Auctions<T>>::take(id).ok_or(Error::<T>::AuctionNotExist)?;
		let current_block_number = frame_system::Pallet::<T>::block_number();
		ensure!(current_block_number < auction.start, Error::<T>::AuctionAlreadyStarted);
		pallet_ajuna_nft::Pallet::<T>::toggle_lock(&auction.owner, auction.token_id).unwrap_or_default();
		<AuctionOwnerById<T>>::remove(id);
		<Auctions<T>>::remove(id);
		Ok(())
	}

	fn bid(bidder: Self::AccountId, id: Self::AuctionId, value: Self::Balance) -> DispatchResult {
		<Auctions<T>>::try_mutate_exists(id, |auction| -> DispatchResult {
			// Basic checks before a bid can be made
			let mut auction = auction.as_mut().ok_or(Error::<T>::AuctionNotExist)?;
			let block_number = <frame_system::Pallet<T>>::block_number();
			ensure!(bidder != auction.owner, Error::<T>::BidOnOwnAuction);
			ensure!(block_number > auction.start, Error::<T>::AuctionNotStarted);
			ensure!(block_number < auction.end, Error::<T>::AuctionAlreadyConcluded);
			ensure!(value >= auction.minimal_bid, Error::<T>::InvalidBidPrice);
			if let Some(ref current_bid) = auction.last_bid {
				ensure!(value > current_bid.1, Error::<T>::InvalidBidPrice);
				// Unlock funds from the previous bid
				T::Currency::remove_lock(AUCTION_LOCK_ID, &current_bid.0);
			} else {
				ensure!(!value.is_zero(), Error::<T>::InvalidBidPrice);
			}
			// Lock funds
			T::Currency::set_lock(AUCTION_LOCK_ID, &bidder, value, WithdrawReasons::all());
			auction.last_bid = Some((bidder, value));
			// Set next minimal bid
			let minimal_bid_step = Permill::from_percent(BID_STEP_PERC).mul_floor(value);
			auction.minimal_bid = value.checked_add(&minimal_bid_step).ok_or(Error::<T>::BidOverflow)?;
			// Avoid auction sniping
			let time_left = auction
				.end
				.checked_sub(&block_number)
				.ok_or(Error::<T>::TimeUnderflow)?;
			if time_left < BID_ADD_BLOCKS.into() {
				auction.end = block_number + BID_ADD_BLOCKS.into();
			}
			Ok(())
		})
	}
}
