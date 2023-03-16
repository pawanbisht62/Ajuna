use crate::{
	types::{
		avatar::{mint::v2::*, types::*},
		Avatar,
	},
	Config,
};

pub(crate) trait AvatarMutator<T: Config> {
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar;
}

fn mutate_dna_strand(dna: &mut Dna, position: usize, byte_type: ByteType, value: u8) {
	match byte_type {
		ByteType::Full => dna[position] = value,
		ByteType::High => dna[position] = (dna[position] & (ByteType::High as u8)) | (value << 4),
		ByteType::Low =>
			dna[position] =
				(dna[position] & (ByteType::Low as u8)) | (value & (ByteType::High as u8)),
	}
}

fn compute_random_quantity(dna: &[u8]) -> u8 {
	(dna[0] & dna[6]) % MAX_QUANTITY
}

fn splice_dna_strands(first_strand: u8, second_strand: u8) -> u16 {
	((first_strand as u16) << 8) | (second_strand as u16)
}

impl<T> AvatarMutator<T> for PetItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		let mut avatar = base_avatar;

		// ItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::High, ItemType::Pet as u8);

		match self {
			PetItemType::Pet => {
				// PetItemType
				mutate_dna_strand(&mut avatar.dna, 0, ByteType::Low, PetItemType::Pet as u8);
				// Quantity
				mutate_dna_strand(&mut avatar.dna, 3, ByteType::Full, 1);
			},
			PetItemType::PetPart => {
				// ItemType
				mutate_dna_strand(&mut avatar.dna, 0, ByteType::Low, PetItemType::PetPart as u8);

				// ClassType1
				mutate_dna_strand(
					&mut avatar.dna,
					1,
					ByteType::High,
					SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
				);
				// ClassType2
				mutate_dna_strand(
					&mut avatar.dna,
					1,
					ByteType::Low,
					SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
				);

				// CustomType1
				mutate_dna_strand(&mut avatar.dna, 2, ByteType::High, HexType::X1 as u8);
				// Quantity
				let quantity = compute_random_quantity(&avatar.dna);
				mutate_dna_strand(&mut avatar.dna, 3, ByteType::Full, quantity);
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

				avatar.souls = quantity as u32 * HexType::X1 as u32;
			},
			PetItemType::Egg => {
				let soul_points = ((avatar.dna[1] ^ avatar.dna[4]) % 99) + 1;
				// PetItemType
				mutate_dna_strand(&mut avatar.dna, 0, ByteType::Low, PetItemType::Egg as u8);
				// CustomType1
				mutate_dna_strand(&mut avatar.dna, 2, ByteType::High, HexType::X0 as u8);
				// CustomType2
				let pet_variation = (avatar.dna[8] & avatar.dna[7]) % 16;
				mutate_dna_strand(&mut avatar.dna, 4, ByteType::Full, pet_variation);
				// Quantity
				mutate_dna_strand(&mut avatar.dna, 3, ByteType::Full, 1);
				// ProgressArray
				// TODO: Finish
				mutate_dna_strand(&mut avatar.dna, 21, ByteType::Full, 1);
				/*

				var result = new Pet(array, PetItemType.Egg)
				{
					RarityType = rarityType,
					CustomType1 = HexType.X0, // Egg is not stackable, no sp ratio
					CustomType2 = petVariation,
					Quantity = 1,

					// add progressbytes here
					ProgressArray = AvatarTools.ProgressBytes(rarityType, Constants.ProgressProbability, array.Skip(21)),
				};

				result.Avatar.SoulPoints = soulpoints;
				return result;
					*/
			},
		}

		avatar
	}
}

impl<T> AvatarMutator<T> for MaterialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		let mut avatar = base_avatar;

		// ItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::High, ItemType::Material as u8);

		// MaterialItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::Low, *self as u8);

		// CustomType1
		mutate_dna_strand(&mut avatar.dna, 2, ByteType::High, HexType::X1 as u8);

		// Quantity
		let quantity = compute_random_quantity(&avatar.dna);
		mutate_dna_strand(&mut avatar.dna, 3, ByteType::Full, quantity);

		avatar.souls = quantity as u32 * HexType::X1 as u32;

		avatar
	}
}

impl<T> AvatarMutator<T> for EssenceItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		let mut avatar = base_avatar;

		// ItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::High, ItemType::Essence as u8);

		// EssenceItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::Low, *self as u8);

		// CustomType1
		mutate_dna_strand(&mut avatar.dna, 2, ByteType::High, HexType::X1 as u8);

		// Quantity
		let quantity = compute_random_quantity(&avatar.dna);
		mutate_dna_strand(&mut avatar.dna, 3, ByteType::Full, quantity);

		avatar.souls = quantity as u32 * HexType::X1 as u32;

		avatar
	}
}

impl<T> AvatarMutator<T> for EquipableItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		let mut avatar = base_avatar;

		mutate_dna_strand(&mut avatar.dna, 0, ByteType::High, ItemType::Equipable as u8);
		/*
				   // for minted equippables add soulpoints
		   if (soulPoints == null)
		   {
			   var sub4Arr2 = new byte[2];
			   Array.Copy(array, 26, sub4Arr2, 0, 2);
			   var randShort4 = BitConverter.ToUInt16(sub4Arr2);
			   soulPoints = (byte)((randShort4 % 25) + 1) * (int)rarityType;
		   }

		   var armorAssembleProgress = AvatarTools.IsArmor(equippableItemType) ? AvatarTools.EnumsToBits(new List<EquippableItemType> { equippableItemType }) : 0;

		   var result = new Equippable(array, equippableItemType)
		   {
			   ClassType1 = (HexType)slotType,
			   ClassType2 = (HexType)petType,
			   RarityType = rarityType,
			   CustomType1 = HexType.X0, // Equippable is not stackable, no sp ratio
			   Quantity = 1,
			   CustomType2 = (byte) HexType.X0, // slot occupation
			   SpecByte1 = (byte)armorAssembleProgress, // make sure that it is properly set
			   SpecByte2 = 0,
			   SpecByte3 = 0,
			   SpecByte4 = 0,
			   SpecByte5 = 0,
			   SpecByte6 = 0,
			   SpecByte7 = 0,
			   SpecByte8 = 0,

			   // add progressbytes here
			   ProgressArray = AvatarTools.ProgressBytes(rarityType, Constants.ProgressProbability, array.Skip(21)),
		   };

		   result.Avatar.SoulPoints = soulPoints.Value;
		   return result;
		*/
		todo!()
	}
}

impl<T> AvatarMutator<T> for BlueprintItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		let mut avatar = base_avatar;

		// ItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::High, ItemType::Blueprint as u8);

		// BlueprintItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::Low, *self as u8);

		// ClassType1
		mutate_dna_strand(
			&mut avatar.dna,
			1,
			ByteType::High,
			SlotRoller::<T>::roll_on(&ARMOR_SLOT_PROBABILITIES) as u8,
		);
		// ClassType2
		mutate_dna_strand(
			&mut avatar.dna,
			1,
			ByteType::Low,
			SlotRoller::<T>::roll_on(&PET_TYPE_PROBABILITIES) as u8,
		);

		// CustomType1
		mutate_dna_strand(&mut avatar.dna, 2, ByteType::High, HexType::X1 as u8);

		// Quantity
		let quantity = ((splice_dna_strands(avatar.dna[26], avatar.dna[27]) % 25) + 1);
		mutate_dna_strand(&mut avatar.dna, 3, ByteType::Full, quantity as u8);

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

		avatar.souls = quantity as SoulCount;

		avatar
	}
}

impl<T> AvatarMutator<T> for SpecialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		let mut avatar = base_avatar;

		// ItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::High, ItemType::Special as u8);

		// SpecialItemType
		mutate_dna_strand(&mut avatar.dna, 0, ByteType::Low, *self as u8);

		// CustomType1
		mutate_dna_strand(&mut avatar.dna, 2, ByteType::High, HexType::X0 as u8);

		// Quantity
		mutate_dna_strand(&mut avatar.dna, 3, ByteType::Full, 1);

		avatar.souls = ((splice_dna_strands(avatar.dna[26], avatar.dna[27]) % 25) + 1) as SoulCount;

		avatar
	}
}
