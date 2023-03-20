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
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar;
}

impl<T> AvatarMutator<T> for PetItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		match self {
			PetItemType::Pet => {
				let quantity = 1;

				AvatarBuilder::with_base_avatar(base_avatar)
					.into_pet(PetItemType::Pet)
					.with_attribute_raw(AvatarAttributes::Quantity, quantity)
					.build()
			},
			PetItemType::PetPart => {
				let quantity =
					MutatorUtils::random_quantity_from_dna_strands(&base_avatar.dna[2..5]);

				AvatarBuilder::with_base_avatar(base_avatar)
					.into_pet(PetItemType::PetPart)
					.with_attribute(
						AvatarAttributes::ClassType1,
						SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES),
					)
					.with_attribute(
						AvatarAttributes::ClassType2,
						SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES),
					)
					.with_attribute(AvatarAttributes::CustomType1, HexType::X1)
					.with_attribute_raw(AvatarAttributes::Quantity, quantity)
					// TODO SpecByte
					.with_spec_byte(AvatarSpecBytes::SpecByte1, 1)
					.with_spec_byte(AvatarSpecBytes::SpecByte2, 1)
					.with_spec_byte(AvatarSpecBytes::SpecByte3, 1)
					.with_spec_byte(AvatarSpecBytes::SpecByte4, 1)
					.with_spec_byte(AvatarSpecBytes::SpecByte5, 1)
					.with_spec_byte(AvatarSpecBytes::SpecByte6, 1)
					.with_spec_byte(AvatarSpecBytes::SpecByte7, 1)
					.with_spec_byte(AvatarSpecBytes::SpecByte8, 1)
					/*
					var baseSeed = (int)petType + (int)slotType;
					var base0 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorBase);
					var comp1 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorComponent1);
					var comp2 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorComponent2);
					var comp3 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorComponent3);
					SpecByte1 = AvatarTools.EnumsToBits(base0),
					SpecByte2 = AvatarTools.EnumsOrderToBits(base0),
					SpecByte3 = AvatarTools.EnumsToBits(comp1),
					SpecByte4 = AvatarTools.EnumsOrderToBits(comp1),
					SpecByte5 = AvatarTools.EnumsToBits(comp2),
					SpecByte6 = AvatarTools.EnumsOrderToBits(comp2),
					SpecByte7 = AvatarTools.EnumsToBits(comp3),
					SpecByte8 = AvatarTools.EnumsOrderToBits(comp3),
					*/
					.with_soul_count(quantity as u32 * HexType::X1 as u32)
					.build()
			},
			PetItemType::Egg => {
				let pet_variation = (base_avatar.dna[8] & base_avatar.dna[7]) % 16;
				let soul_count = ((base_avatar.dna[1] ^ base_avatar.dna[4]) % 99) as SoulCount + 1;

				AvatarBuilder::with_base_avatar(base_avatar)
					.into_pet(PetItemType::Egg)
					// TODO
					.with_attribute_raw(AvatarAttributes::RarityType, 1)
					.with_attribute(AvatarAttributes::CustomType1, HexType::X0)
					.with_attribute_raw(AvatarAttributes::CustomType2, pet_variation)
					.with_attribute_raw(AvatarAttributes::Quantity, 1)
					// TODO
					.with_progress_array([0; 11])
					/*
					Parameter -> rarityType
					{
						ProgressArray = AvatarTools.ProgressBytes(rarityType, Constants.ProgressProbability, array.Skip(21)),
					};
					*/
					// Soul points
					.with_soul_count(soul_count)
					.build()
			},
		}
	}
}

impl<T> AvatarMutator<T> for MaterialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		let quantity = MutatorUtils::random_quantity_from_dna_strands(&base_avatar.dna[0..3]);

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_material(*self)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X1)
			.with_attribute_raw(AvatarAttributes::Quantity, quantity)
			.with_soul_count(quantity as u32 * HexType::X1 as u32)
			.build()
	}
}

impl<T> AvatarMutator<T> for EssenceItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		let quantity = MutatorUtils::random_quantity_from_dna_strands(&base_avatar.dna[5..9]);

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_essence(*self)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X1)
			.with_attribute_raw(AvatarAttributes::Quantity, quantity)
			.with_soul_count(quantity as u32 * HexType::X1 as u32)
			.build()
	}
}

impl<T> AvatarMutator<T> for EquipableItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_equipable(*self)
			.with_attribute(
				AvatarAttributes::ClassType1,
				SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES),
			)
			.with_attribute(
				AvatarAttributes::ClassType2,
				SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES),
			)
			// TODO RarityType
			.with_attribute_raw(AvatarAttributes::RarityType, 1)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X0)
			.with_attribute(AvatarAttributes::CustomType2, HexType::X0)
			.with_attribute_raw(AvatarAttributes::Quantity, 1)
			// TODO SpecByte and ProgressArray
			.with_spec_byte(AvatarSpecBytes::SpecByte1, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte2, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte3, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte4, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte5, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte6, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte7, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte8, 1)
			.with_progress_array([0; 11])
			/*
			var armorAssembleProgress = AvatarTools.IsArmor(equippableItemType) ? AvatarTools.EnumsToBits(new List<EquippableItemType> { equippableItemType }) : 0;
			var result = new Equippable(array, equippableItemType)
			{
				RarityType = rarityType,
				SpecByte1 = (byte)armorAssembleProgress, // make sure that it is properly set
				// add progressbytes here
				ProgressArray = AvatarTools.ProgressBytes(rarityType, Constants.ProgressProbability, array.Skip(21)),
			};
			*/
			.with_soul_count(((spliced_dna % 25) + 1) as SoulCount)
			.build()
	}
}

impl<T> AvatarMutator<T> for BlueprintItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);
		let quantity = ((spliced_dna % 25) + 1) as u8;

		AvatarBuilder::with_base_avatar(base_avatar)
			.into_blueprint(*self)
			.with_attribute(
				AvatarAttributes::ClassType1,
				SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES),
			)
			.with_attribute(
				AvatarAttributes::ClassType2,
				SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES),
			)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X1)
			.with_attribute_raw(AvatarAttributes::Quantity, quantity)
			// TODO SpecByte
			.with_spec_byte(AvatarSpecBytes::SpecByte1, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte2, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte3, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte4, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte5, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte6, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte7, 1)
			/*
			Parameters: pattern, equippableItemType
			// This is a default for now
			var matReq1 = 1;
			var matReq2 = 1;
			var matReq3 = 1;
			var matReq4 = 1;
			SpecByte1 = AvatarTools.EnumsToBits(pattern),
			SpecByte2 = AvatarTools.EnumsOrderToBits(pattern),
			SpecByte3 = (byte)equippableItemType,
			SpecByte4 = (byte)matReq1,
			SpecByte5 = (byte)matReq2,
			SpecByte6 = (byte)matReq3,
			SpecByte7 = (byte)matReq4,
			*/
			.with_soul_count(quantity as SoulCount)
			.build()
	}
}

impl<T> AvatarMutator<T> for SpecialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
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
