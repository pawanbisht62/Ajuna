use super::*;
use crate::types::Avatar;

struct MutatorUtils;

impl MutatorUtils {
	pub fn random_quantity_from_dna_strands(strands: &[u8]) -> u8 {
		strands.iter().fold(0_u8, |acc, x| acc ^ (x << 2)) % MAX_QUANTITY
	}

	pub fn splice_dna_strands(first_strand: u8, second_strand: u8) -> u16 {
		((first_strand as u16) << 8) | (second_strand as u16)
	}
}

pub(crate) trait AvatarMutator<T: Config> {
	fn mutate_from_base(
		&self,
		base_avatar: Avatar,
		hash_provider: &mut HashProvider<T, 32>,
	) -> Avatar;
}

impl<T> AvatarMutator<T> for PetItemType
where
	T: Config,
{
	fn mutate_from_base(
		&self,
		mut base_avatar: Avatar,
		hash_provider: &mut HashProvider<T, 32>,
	) -> Avatar {
		match self {
			PetItemType::Pet =>
				AvatarBuilder::with_base_avatar(base_avatar).into_pet(PetItemType::Pet).build(),
			PetItemType::PetPart => {
				let quantity = hash_provider.get_hash_byte();
				let slot_type = SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES, hash_provider);
				let pet_type = SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES, hash_provider);

				AvatarBuilder::with_base_avatar(base_avatar)
					.into_pet_part(pet_type, slot_type, quantity)
					.build()
			},
			PetItemType::Egg => {
				// TODO
				let rarity_type = RarityType::Common;
				let pet_variation = PetType::FireDino;

				AvatarBuilder::with_base_avatar(base_avatar)
					.into_egg(rarity_type, pet_variation)
					.build()
			},
		}
	}
}

impl<T> AvatarMutator<T> for MaterialItemType
where
	T: Config,
{
	fn mutate_from_base(
		&self,
		mut base_avatar: Avatar,
		hash_provider: &mut HashProvider<T, 32>,
	) -> Avatar {
		let quantity = MutatorUtils::random_quantity_from_dna_strands(&base_avatar.dna[0..3]);

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_material(*self, quantity)
			.build()
	}
}

impl<T> AvatarMutator<T> for EssenceItemType
where
	T: Config,
{
	fn mutate_from_base(
		&self,
		mut base_avatar: Avatar,
		hash_provider: &mut HashProvider<T, 32>,
	) -> Avatar {
		AvatarBuilder::with_base_avatar(base_avatar).into_essence(*self, 1).build()
	}
}

impl<T> AvatarMutator<T> for EquipableItemType
where
	T: Config,
{
	fn mutate_from_base(
		&self,
		mut base_avatar: Avatar,
		hash_provider: &mut HashProvider<T, 32>,
	) -> Avatar {
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);

		let pet_type = SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES, hash_provider);
		let slot_type = SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES, hash_provider);

		// TODO
		let rarity_type = RarityType::Uncommon;

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_equipable(*self, pet_type, slot_type, rarity_type, spliced_dna as SoulCount)
			.build()
	}
}

impl<T> AvatarMutator<T> for BlueprintItemType
where
	T: Config,
{
	fn mutate_from_base(
		&self,
		mut base_avatar: Avatar,
		hash_provider: &mut HashProvider<T, 32>,
	) -> Avatar {
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);
		let quantity = ((spliced_dna % 25) + 1) as u8;

		let pet_type = SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES, hash_provider);
		let slot_type = SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES, hash_provider);

		// TODO
		let pattern = vec![];
		// TODO
		let equipable_item_type = EquipableItemType::ArmorBase;

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_blueprint(
				*self,
				pet_type,
				slot_type,
				equipable_item_type,
				pattern,
				quantity as SoulCount,
			)
			.build()
	}
}

impl<T> AvatarMutator<T> for SpecialItemType
where
	T: Config,
{
	fn mutate_from_base(
		&self,
		mut base_avatar: Avatar,
		hash_provider: &mut HashProvider<T, 32>,
	) -> Avatar {
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_special(*self)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X0)
			.with_attribute_raw(AvatarAttributes::Quantity, 1)
			.with_soul_count(((spliced_dna % 25) + 1) as SoulCount)
			.build()
	}
}
