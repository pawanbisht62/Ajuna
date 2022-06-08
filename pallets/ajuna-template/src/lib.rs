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
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	sp_runtime::traits::{Dispatchable, Hash, TrailingZeroInput},
	traits::{
		schedule::{DispatchTime, Named},
		LockIdentifier, Randomness,
	},
};
use scale_info::TypeInfo;

use sp_std::prelude::*;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Implementations of some helper traits passed into runtime modules as associated types.
pub mod logic;
use logic::Logic;

const TEMPLATE_ID: LockIdentifier = *b"template";


/// Connect four board structure containing two players and the board
#[derive(Encode, Decode, Clone, PartialEq, MaxEncodedLen, Debug, TypeInfo)]
pub struct TemplateStruct<Hash, AccountId, BlockNumber> {
	pub id: Hash,
	pub account: AccountId,
	pub blocknumber: BlockNumber,
}

const TEMP_1: u8 = 1;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult, pallet_prelude::*, sp_runtime::traits::TrailingZeroInput,
	};
	use frame_system::pallet_prelude::*;

	// important to use outside structs and consts
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Proposal: Parameter + Dispatchable<Origin = Self::Origin> + From<Call<Self>>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The generator used to supply randomness to contracts through `seal_random`.
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		// /// Weight information for extrinsics in this pallet.
		//type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn founder_key)]
	pub type FounderKey<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn boards)]
	/// Store all templates that are currently being played.
	pub type Templates<T: Config> = StorageMap<
		_,
		Identity,
		T::Hash,
		TemplateStruct<T::Hash, T::AccountId, T::BlockNumber>,
		OptionQuery,
	>;

	// Default value for Nonce
	#[pallet::type_value]
	pub fn NonceDefault() -> u64 {
		0
	}
	// Nonce used for generating a different seed each time.
	#[pallet::storage]
	pub type Nonce<T: Config> = StorageValue<_, u64, ValueQuery, NonceDefault>;

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
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
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
		fn on_finalize(_n: BlockNumberFor<T>) {
			// Perform necessary data/state clean up here.
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

	}
}

impl<T: Config> Pallet<T> {
	/// Update nonce once used.
	fn encode_and_update_nonce() -> Vec<u8> {
		let nonce = <Nonce<T>>::get();
		<Nonce<T>>::put(nonce.wrapping_add(1));
		nonce.encode()
	}

	/// Generates a random hash out of a seed.
	fn generate_random_hash(phrase: &[u8], sender: T::AccountId) -> T::Hash {
		let (seed, _) = T::Randomness::random(phrase);
		let seed = <[u8; 32]>::decode(&mut TrailingZeroInput::new(seed.as_ref()))
			.expect("input is padded with zeroes; qed");
		(seed, &sender, Self::encode_and_update_nonce()).using_encoded(T::Hashing::hash)
	}
}
