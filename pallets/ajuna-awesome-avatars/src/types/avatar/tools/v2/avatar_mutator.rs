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
		// ItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemType,
			ItemType::Pet as u8,
		);

		match self {
			PetItemType::Pet => {
				// PetItemType
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::ItemSubType,
					PetItemType::Pet as u8,
				);
				// Quantity
				AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::Quantity, 1);
			},
			PetItemType::PetPart => {
				// PetItemType
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::ItemSubType,
					PetItemType::PetPart as u8,
				);
				// ClassType1
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::ClassType1,
					SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
				);
				// ClassType2
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::ClassType2,
					SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
				);
				// CustomType1
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::CustomType1,
					HexType::X1 as u8,
				);
				// Quantity
				let quantity =
					MutatorUtils::random_quantity_from_dna_strands(&base_avatar.dna[2..5]);
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::Quantity,
					quantity,
				);
				// SpecByte 1
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte1, 1);
				// SpecByte 2
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte2, 1);
				// SpecByte 3
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte3, 1);
				// SpecByte 4
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte4, 1);
				// SpecByte 5
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte5, 1);
				// SpecByte 6
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte6, 1);
				// SpecByte 7
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte7, 1);
				// SpecByte 8
				// TODO
				AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte8, 1);
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

				base_avatar.souls = quantity as u32 * HexType::X1 as u32;
			},
			PetItemType::Egg => {
				// PetItemType
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::ItemSubType,
					PetItemType::Egg as u8,
				);
				// RarityType
				// TODO
				AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::RarityType, 1);
				// CustomType1
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::CustomType1,
					HexType::X0 as u8,
				);
				// CustomType2
				let pet_variation = (base_avatar.dna[8] & base_avatar.dna[7]) % 16;
				AvatarWrapper::write_attribute(
					&mut base_avatar,
					AvatarAttributes::CustomType2,
					pet_variation,
				);
				// Quantity
				AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::Quantity, 1);
				// ProgressArray
				// TODO
				AvatarWrapper::write_progress_array(&mut base_avatar, [0; 11]);
				/*
				Parameter -> rarityType
				{
					ProgressArray = AvatarTools.ProgressBytes(rarityType, Constants.ProgressProbability, array.Skip(21)),
				};
				*/
				// Soul points
				base_avatar.souls =
					((base_avatar.dna[1] ^ base_avatar.dna[4]) % 99) as SoulCount + 1;
			},
		}

		base_avatar
	}
}

impl<T> AvatarMutator<T> for MaterialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		// ItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemType,
			ItemType::Material as u8,
		);
		// MaterialItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemSubType,
			*self as u8,
		);
		// CustomType1
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::CustomType1,
			HexType::X1 as u8,
		);
		// Quantity
		let quantity = MutatorUtils::random_quantity_from_dna_strands(&base_avatar.dna[0..3]);
		AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::Quantity, quantity);
		// Soul points
		base_avatar.souls = quantity as u32 * HexType::X1 as u32;

		base_avatar
	}
}

impl<T> AvatarMutator<T> for EssenceItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		// ItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemType,
			ItemType::Essence as u8,
		);
		// MaterialItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemSubType,
			*self as u8,
		);
		// CustomType1
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::CustomType1,
			HexType::X1 as u8,
		);
		// Quantity
		let quantity = MutatorUtils::random_quantity_from_dna_strands(&base_avatar.dna[5..9]);
		AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::Quantity, quantity);
		// Soul points
		base_avatar.souls = quantity as u32 * HexType::X1 as u32;

		base_avatar
	}
}

impl<T> AvatarMutator<T> for EquipableItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		// ItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemType,
			ItemType::Equipable as u8,
		);
		// EquipableItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemSubType,
			*self as u8,
		);
		// ClassType1
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ClassType1,
			SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
		);
		// ClassType2
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ClassType2,
			SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
		);
		// RarityType
		// TODO
		AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::RarityType, 1);
		// CustomType1
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::CustomType1,
			HexType::X0 as u8,
		);
		// CustomType2
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::CustomType2,
			HexType::X0 as u8,
		);
		// Quantity
		AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::Quantity, 1);
		// SpecByte 1
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte1, 1);
		// SpecByte 2
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte2, 0);
		// SpecByte 3
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte3, 0);
		// SpecByte 4
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte4, 0);
		// SpecByte 5
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte5, 0);
		// SpecByte 6
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte6, 0);
		// SpecByte 7
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte7, 0);
		// SpecByte 8
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte7, 0);
		// ProgressArray
		// TODO
		AvatarWrapper::write_progress_array(&mut base_avatar, [0; 11]);
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
		// Soul points
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);
		base_avatar.souls = ((spliced_dna % 25) + 1) as SoulCount;
		base_avatar
	}
}

impl<T> AvatarMutator<T> for BlueprintItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		// ItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemType,
			ItemType::Blueprint as u8,
		);
		// BlueprintItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemSubType,
			*self as u8,
		);
		// ClassType1
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ClassType1,
			SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
		);
		// ClassType2
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ClassType2,
			SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
		);
		// CustomType1
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::CustomType1,
			HexType::X1 as u8,
		);
		// Quantity
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);
		let quantity = (spliced_dna % 25) + 1;
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::Quantity,
			quantity as u8,
		);
		// SpecByte 1
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte1, 1);
		// SpecByte 2
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte2, 1);
		// SpecByte 3
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte3, 1);
		// SpecByte 4
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte4, 1);
		// SpecByte 5
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte5, 1);
		// SpecByte 6
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte6, 1);
		// SpecByte 7
		// TODO
		AvatarWrapper::write_spec_byte(&mut base_avatar, AvatarSpecBytes::SpecByte7, 1);
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
		// Soul points
		base_avatar.souls = quantity as SoulCount;

		base_avatar
	}
}

impl<T> AvatarMutator<T> for SpecialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, mut base_avatar: Avatar) -> Avatar {
		// ItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemType,
			ItemType::Special as u8,
		);
		// SpecialItemType
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::ItemSubType,
			*self as u8,
		);
		// CustomType1
		AvatarWrapper::write_attribute(
			&mut base_avatar,
			AvatarAttributes::CustomType1,
			HexType::X0 as u8,
		);
		// Quantity
		AvatarWrapper::write_attribute(&mut base_avatar, AvatarAttributes::Quantity, 1);
		// Soul points
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);
		base_avatar.souls = ((spliced_dna % 25) + 1) as SoulCount;

		base_avatar
	}
}
