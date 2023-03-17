use crate::{
	types::{
		avatar::{mint::v2::*, types::*},
		Avatar,
	},
	Config,
};

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
		let mut avatar = WrappedAvatar::from(&mut base_avatar);

		// ItemType
		avatar.write_attribute(AvatarAttributes::ItemType, ItemType::Pet as u8);

		match self {
			PetItemType::Pet => {
				// PetItemType
				avatar.write_attribute(AvatarAttributes::ItemSubType, PetItemType::Pet as u8);
				// Quantity
				avatar.write_attribute(AvatarAttributes::Quantity, 1);
			},
			PetItemType::PetPart => {
				// PetItemType
				avatar.write_attribute(AvatarAttributes::ItemSubType, PetItemType::PetPart as u8);
				// ClassType1
				avatar.write_attribute(
					AvatarAttributes::ClassType1,
					SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
				);
				// ClassType2
				avatar.write_attribute(
					AvatarAttributes::ClassType2,
					SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
				);
				// CustomType1
				avatar.write_attribute(AvatarAttributes::CustomType1, HexType::X1 as u8);
				// Quantity
				let quantity =
					MutatorUtils::random_quantity_from_dna_strands(&avatar.inner.dna[2..5]);
				avatar.write_attribute(AvatarAttributes::Quantity, quantity);
				// SpecByte 1
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte1, 1);
				// SpecByte 2
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte2, 1);
				// SpecByte 3
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte3, 1);
				// SpecByte 4
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte4, 1);
				// SpecByte 5
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte5, 1);
				// SpecByte 6
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte6, 1);
				// SpecByte 7
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte7, 1);
				// SpecByte 8
				// TODO
				avatar.write_spec_byte(AvatarSpecBytes::SpecByte8, 1);
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
				avatar.write_attribute(AvatarAttributes::ItemSubType, PetItemType::Egg as u8);
				// RarityType
				// TODO
				avatar.write_attribute(AvatarAttributes::RarityType, 1);
				// CustomType1
				avatar.write_attribute(AvatarAttributes::CustomType1, HexType::X0 as u8);
				// CustomType2
				let pet_variation = (avatar.inner.dna[8] & avatar.inner.dna[7]) % 16;
				avatar.write_attribute(AvatarAttributes::CustomType2, pet_variation);
				// Quantity
				avatar.write_attribute(AvatarAttributes::Quantity, 1);
				// ProgressArray
				// TODO
				avatar.write_progress_array([0; 11]);
				/*
				Parameter -> rarityType
				{
					ProgressArray = AvatarTools.ProgressBytes(rarityType, Constants.ProgressProbability, array.Skip(21)),
				};
				*/
				// Soul points
				base_avatar.souls =
					((avatar.inner.dna[1] ^ avatar.inner.dna[4]) % 99) as SoulCount + 1;
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
		let mut avatar = WrappedAvatar::from(&mut base_avatar);

		// ItemType
		avatar.write_attribute(AvatarAttributes::ItemType, ItemType::Material as u8);
		// MaterialItemType
		avatar.write_attribute(AvatarAttributes::ItemSubType, *self as u8);
		// CustomType1
		avatar.write_attribute(AvatarAttributes::CustomType1, HexType::X1 as u8);
		// Quantity
		let quantity = MutatorUtils::random_quantity_from_dna_strands(&avatar.inner.dna[0..3]);
		avatar.write_attribute(AvatarAttributes::Quantity, quantity);
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
		let mut avatar = WrappedAvatar::from(&mut base_avatar);

		// ItemType
		avatar.write_attribute(AvatarAttributes::ItemType, ItemType::Essence as u8);
		// MaterialItemType
		avatar.write_attribute(AvatarAttributes::ItemSubType, *self as u8);
		// CustomType1
		avatar.write_attribute(AvatarAttributes::CustomType1, HexType::X1 as u8);
		// Quantity
		let quantity = MutatorUtils::random_quantity_from_dna_strands(&avatar.inner.dna[5..9]);
		avatar.write_attribute(AvatarAttributes::Quantity, quantity);
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
		let mut avatar = WrappedAvatar::from(&mut base_avatar);

		// ItemType
		avatar.write_attribute(AvatarAttributes::ItemType, ItemType::Equipable as u8);
		// EquipableItemType
		avatar.write_attribute(AvatarAttributes::ItemSubType, *self as u8);
		// ClassType1
		avatar.write_attribute(
			AvatarAttributes::ClassType1,
			SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
		);
		// ClassType2
		avatar.write_attribute(
			AvatarAttributes::ClassType2,
			SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
		);
		// RarityType
		// TODO
		avatar.write_attribute(AvatarAttributes::RarityType, 1);
		// CustomType1
		avatar.write_attribute(AvatarAttributes::CustomType1, HexType::X0 as u8);
		// CustomType2
		avatar.write_attribute(AvatarAttributes::CustomType2, HexType::X0 as u8);
		// Quantity
		avatar.write_attribute(AvatarAttributes::Quantity, 1);
		// SpecByte 1
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte1, 1);
		// SpecByte 2
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte2, 0);
		// SpecByte 3
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte3, 0);
		// SpecByte 4
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte4, 0);
		// SpecByte 5
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte5, 0);
		// SpecByte 6
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte6, 0);
		// SpecByte 7
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte7, 0);
		// SpecByte 8
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte7, 0);
		// ProgressArray
		// TODO
		avatar.write_progress_array([0; 11]);
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
		let mut avatar = WrappedAvatar::from(&mut base_avatar);

		// ItemType
		avatar.write_attribute(AvatarAttributes::ItemType, ItemType::Blueprint as u8);
		// BlueprintItemType
		avatar.write_attribute(AvatarAttributes::ItemSubType, *self as u8);
		// ClassType1
		avatar.write_attribute(
			AvatarAttributes::ClassType1,
			SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
		);
		// ClassType2
		avatar.write_attribute(
			AvatarAttributes::ClassType2,
			SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
		);
		// CustomType1
		avatar.write_attribute(AvatarAttributes::CustomType1, HexType::X1 as u8);
		// Quantity
		let spliced_dna =
			MutatorUtils::splice_dna_strands(avatar.inner.dna[26], avatar.inner.dna[27]);
		let quantity = (spliced_dna % 25) + 1;
		avatar.write_attribute(AvatarAttributes::Quantity, quantity as u8);
		// SpecByte 1
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte1, 1);
		// SpecByte 2
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte2, 1);
		// SpecByte 3
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte3, 1);
		// SpecByte 4
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte4, 1);
		// SpecByte 5
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte5, 1);
		// SpecByte 6
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte6, 1);
		// SpecByte 7
		// TODO
		avatar.write_spec_byte(AvatarSpecBytes::SpecByte7, 1);
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
		let mut avatar = WrappedAvatar::from(&mut base_avatar);

		// ItemType
		avatar.write_attribute(AvatarAttributes::ItemType, ItemType::Special as u8);
		// SpecialItemType
		avatar.write_attribute(AvatarAttributes::ItemSubType, *self as u8);
		// CustomType1
		avatar.write_attribute(AvatarAttributes::CustomType1, HexType::X0 as u8);
		// Quantity
		avatar.write_attribute(AvatarAttributes::Quantity, 1);
		// Soul points
		let spliced_dna =
			MutatorUtils::splice_dna_strands(base_avatar.dna[26], base_avatar.dna[27]);
		base_avatar.souls = ((spliced_dna % 25) + 1) as SoulCount;

		base_avatar
	}
}
