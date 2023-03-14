mod constants;

use crate::{
	types::avatar::{mint::v2::constants::*, types::ItemType},
	*,
};
use sp_runtime::DispatchError;

pub(crate) struct AvatarMinterV2<'a, T: Config>(pub PhantomData<&'a T>);

#[allow(unused_variables)]
impl<'a, T> Minter<T> for AvatarMinterV2<'a, T>
where
	T: Config,
{
	fn mint_avatar_set(
		&self,
		player: &T::AccountId,
		season_id: &SeasonId,
		season: &SeasonOf<T>,
		mint_option: &MintOption,
	) -> Result<Vec<MintOutput<T>>, DispatchError> {
		let rolled_item_type = self.roll_for_item_type(mint_option.mint_pack);

		// After that roll as many times as needed for the specific subtypes, use the mint_options
		// MintPackSize for this. Then for each item call the corresponding function to generate it,
		// that or just have a generic function an alter the DNA

		todo!()
	}
}

impl<'a, T> AvatarMinterV2<'a, T>
where
	T: Config,
{
	/// Rolls number between 1 and 1000, representing a range of 0.1% increments in probability.
	fn roll_number(&self) -> u32 {
		let (random_hash, _) = T::Randomness::random(b"roll");

		// TODO: Improve random generation logic
		(u32::from_ne_bytes(random_hash.as_ref()[0..4].try_into().unwrap_or_default()) % 1000) + 1
	}

	fn roll_on<S>(&self, slots: &[(S, u32)]) -> S
	where
		S: Copy + Clone + Default,
	{
		let mut item_rolled = S::default();
		let mut roll = self.roll_number();

		for (slot_item, slot_probability) in slots {
			roll = roll.saturating_sub(*slot_probability);

			if roll == 0 {
				item_rolled = *slot_item;
				break
			}
		}

		item_rolled
	}

	fn roll_for_item_type(&self, pack_type: PackType) -> ItemType {
		let slots = match pack_type {
			PackType::Material => PACK_TYPE_MATERIAL_ITEM_PROBABILITIES,
			PackType::Equipment => PACK_TYPE_EQUIPMENT_ITEM_PROBABILITIES,
			PackType::Special => PACK_TYPE_SPECIAL_ITEM_PROBABILITIES,
		};

		self.roll_on::<ItemType>(&slots)
	}

	// TODO: Add roll functions for each ItemType category
}
