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
	dispatch::{DispatchError, DispatchResult},
	sp_runtime::traits::{StaticLookup, Zero},
	ensure,
};
use scale_info::TypeInfo;

use sp_std::prelude::*;

mod benchmarking;

pub mod weights;
use weights::WeightInfo;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type Balance = u128;
pub type ClassData = u32;
pub type TokenIdOf<T> = <T as orml_nft::Config>::TokenId;
pub type ClassIdOf<T> = <T as orml_nft::Config>::ClassId;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, Debug, TypeInfo)]
pub struct TokenData {
	pub locked: bool,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;

	// important to use outside structs and consts
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + orml_nft::Config<ClassData = ClassData, TokenData = TokenData> {

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NFTTokenClassCreated(T::AccountId, T::ClassId),
		NFTTokenMinted(T::AccountId, T::ClassId, u32),
		NFTTokenMintedLockToggled(T::AccountId, T::ClassId, T::TokenId, bool),
		NFTTokenTransferred(T::AccountId, T::AccountId, T::ClassId, T::TokenId),
		NFTTokenBurned(T::AccountId, T::ClassId, T::TokenId),
		NFTTokenClassDestroyed(T::AccountId, T::ClassId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ClassNotFound,
		TokenNotFound,
		NoPermission,
		CannotDestroyClass,
		TokenLocked,
		InvalidQuantity,
	}

	// Pallet implements [`Hooks`] trait to define some logic to execute in some context.
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> { }

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(<T as Config>::WeightInfo::create_class())]
		pub fn create_class(origin: OriginFor<T>, metadata: Vec<u8>, data: T::ClassData) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let class_id = orml_nft::Pallet::<T>::create_class(&sender, metadata, data)?;
			Self::deposit_event(Event::NFTTokenClassCreated(sender, class_id));
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::mint())]
		pub fn mint(
			origin: OriginFor<T>,
			class_id: <T as orml_nft::Config>::ClassId,
			metadata: Vec<u8>,
			token_data: TokenData,
			quantity: u32,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(quantity > Zero::zero(), Error::<T>::InvalidQuantity);
			let class_info = orml_nft::Pallet::<T>::classes(class_id).ok_or(Error::<T>::ClassNotFound)?;
			ensure!(sender == class_info.owner, Error::<T>::NoPermission);
			let mut data = token_data;
			data.locked = false;
			for _ in 0..quantity {
				orml_nft::Pallet::<T>::mint(&sender, class_id, metadata.clone(), data.clone())?;
			}
			Self::deposit_event(Event::NFTTokenMinted(sender, class_id, quantity));
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			dest: <T::Lookup as StaticLookup>::Source,
			token: (T::ClassId, T::TokenId),
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let _class_info = orml_nft::Pallet::<T>::classes(token.0).ok_or(Error::<T>::ClassNotFound)?;
			let token_info = orml_nft::Pallet::<T>::tokens(token.0, token.1).ok_or(Error::<T>::TokenNotFound)?;
			ensure!(sender == token_info.owner, Error::<T>::NoPermission);
			ensure!(!token_info.data.locked, Error::<T>::TokenLocked);
			let to: T::AccountId = T::Lookup::lookup(dest)?;
			orml_nft::Pallet::<T>::transfer(&sender, &to, token)?;
			Self::deposit_event(Event::NFTTokenTransferred(sender, to, token.0, token.1));
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::burn())]
		pub fn burn(origin: OriginFor<T>, token: (T::ClassId, T::TokenId)) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let _class_info = orml_nft::Pallet::<T>::classes(token.0).ok_or(Error::<T>::ClassNotFound)?;
			let token_info = orml_nft::Pallet::<T>::tokens(token.0, token.1).ok_or(Error::<T>::TokenNotFound)?;
			ensure!(sender == token_info.owner, Error::<T>::NoPermission);
			ensure!(!token_info.data.locked, Error::<T>::TokenLocked);
			orml_nft::Pallet::<T>::burn(&sender, token)?;
			Self::deposit_event(Event::NFTTokenBurned(sender, token.0, token.1));
			Ok(().into())
		}

		#[pallet::weight(<T as Config>::WeightInfo::destroy_class())]
		pub fn destroy_class(origin: OriginFor<T>, class_id: T::ClassId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let class_info = orml_nft::Pallet::<T>::classes(class_id).ok_or(Error::<T>::ClassNotFound)?;
			ensure!(sender == class_info.owner, Error::<T>::NoPermission);
			ensure!(
				class_info.total_issuance == Zero::zero(),
				Error::<T>::CannotDestroyClass
			);
			orml_nft::Pallet::<T>::destroy_class(&sender, class_id)?;
			Self::deposit_event(Event::NFTTokenClassDestroyed(sender, class_id));
			Ok(().into())
		}

	}
}

impl<T: Config> Pallet<T> {
	pub fn is_owner(account: &T::AccountId, token: (T::ClassId, T::TokenId)) -> bool {
		orml_nft::Pallet::<T>::is_owner(account, token)
	}

	pub fn is_locked(token: (T::ClassId, T::TokenId)) -> Result<bool, DispatchError> {
		let token_info = orml_nft::Pallet::<T>::tokens(token.0, token.1).ok_or(Error::<T>::TokenNotFound)?;
		Ok(token_info.data.locked)
	}

	pub fn toggle_lock(account: &T::AccountId, token_id: (T::ClassId, T::TokenId)) -> DispatchResult {
		let _class_info = orml_nft::Pallet::<T>::classes(token_id.0).ok_or(Error::<T>::ClassNotFound)?;
		orml_nft::Tokens::<T>::mutate_exists(token_id.0, token_id.1, |token| -> DispatchResult {
			if let Some(ref mut token) = token {
				ensure!(*account == token.owner, Error::<T>::NoPermission);
				token.data.locked ^= true; // Toggle
						   // fix clone
				Self::deposit_event(Event::NFTTokenMintedLockToggled(
					account.clone(),
					token_id.0,
					token_id.1,
					token.data.locked,
				));
			}
			Ok(())
		})?;
		Ok(())
	}
}
