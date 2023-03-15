use crate::{
	types::{avatar::types::*, Avatar},
	Config,
};

pub(crate) trait AvatarMutator<T: Config> {
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar;
}

impl<T> AvatarMutator<T> for PetItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		match self {
			PetItemType::Pet => {
				/*
				var result = new Pet(array, PetItemType.Pet)
				{
					Quantity = (byte)1,
				};
				return result;
					 */
			},
			PetItemType::PetPart => {
				/*
				var baseSeed = (int)petType + (int)slotType;
				var base0 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorBase);
				var comp1 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorComponent1);
				var comp2 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorComponent2);
				var comp3 = AvatarTools.CreatePattern<NibbleType>(baseSeed, (int)EquippableItemType.ArmorComponent3);

				var result = new Pet(array, PetItemType.PetPart)
				{
					ClassType1 = (HexType)slotType,
					ClassType2 = (HexType)petType,

					CustomType1 = HexType.X1, // PetPart is stackable, sp ratio set 1
					Quantity = (byte)quantity,

					SpecByte1 = AvatarTools.EnumsToBits(base0),
					SpecByte2 = AvatarTools.EnumsOrderToBits(base0),

					SpecByte3 = AvatarTools.EnumsToBits(comp1),
					SpecByte4 = AvatarTools.EnumsOrderToBits(comp1),

					SpecByte5 = AvatarTools.EnumsToBits(comp2),
					SpecByte6 = AvatarTools.EnumsOrderToBits(comp2),

					SpecByte7 = AvatarTools.EnumsToBits(comp3),
					SpecByte8 = AvatarTools.EnumsOrderToBits(comp3),
				};

				// stackable avatars have to follow the sp ratio regulations
				result.Avatar.SoulPoints = quantity * (int)result.CustomType1;
				return result;
					*/
			},
			PetItemType::Egg => {
				/*
				var sub4Arr2 = new byte[2];
				Array.Copy(array, 26, sub4Arr2, 0, 2);
				var randShort4 = BitConverter.ToUInt16(sub4Arr2);
				var soulpoints = (byte)((randShort4 % 99) + 1);

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
		todo!()
	}
}

impl<T> AvatarMutator<T> for MaterialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		/*
				   var result = new Material(array, materialItemType)
		   {
			   CustomType1 = HexType.X1, // PetPart is stackable, sp ratio set 1
			   Quantity = (byte)quantity,
		   };

		   // stackable avatars have to follow the sp ratio regulations
		   result.Avatar.SoulPoints = quantity * (int)result.CustomType1;
		   return result;
		*/
		match self {
			MaterialItemType::Polymers => {},
			MaterialItemType::Electronics => {},
			MaterialItemType::PowerCells => {},
			MaterialItemType::Optics => {},
			MaterialItemType::Metals => {},
			MaterialItemType::Ceramics => {},
			MaterialItemType::Superconductors => {},
			MaterialItemType::Nanomaterials => {},
		}
		todo!()
	}
}

impl<T> AvatarMutator<T> for EssenceItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		/*
				   var result = new Material(array, materialItemType)
		   {
			   CustomType1 = HexType.X1, // PetPart is stackable, sp ratio set 1
			   Quantity = (byte)quantity,
		   };

		   // stackable avatars have to follow the sp ratio regulations
		   result.Avatar.SoulPoints = quantity * (int)result.CustomType1;
		   return result;
		*/
		todo!()
	}
}

impl<T> AvatarMutator<T> for EquipableItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
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
		/*
			// TODO: add a quantity algorithm
		   var matReq1 = 1;
		   var matReq2 = 1;
		   var matReq3 = 1;
		   var matReq4 = 1;

		   var result = new Blueprint(array, HexType.X1)
		   {
			   ClassType1 = (HexType)slotType,
			   ClassType2 = (HexType)petType,

			   CustomType1 = HexType.X1, // BluePrint is stackable, sp ratio set 1
			   Quantity = soulPoints,

			   SpecByte1 = AvatarTools.EnumsToBits(pattern),
			   SpecByte2 = AvatarTools.EnumsOrderToBits(pattern),
			   SpecByte3 = (byte)equippableItemType,
			   SpecByte4 = (byte)matReq1,
			   SpecByte5 = (byte)matReq2,
			   SpecByte6 = (byte)matReq3,
			   SpecByte7 = (byte)matReq4,
		   };

		   // crafted items always inherite the soulpoints
		   result.Avatar.SoulPoints = soulPoints;
		   return result;
		*/
		todo!()
	}
}

impl<T> AvatarMutator<T> for SpecialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_avatar: Avatar) -> Avatar {
		/*
				   // for minted special add soulpoints
		   if (soulPoints == null)
		   {
			   var sub4Arr2 = new byte[2];
			   Array.Copy(array, 26, sub4Arr2, 0, 2);
			   var randShort4 = BitConverter.ToUInt16(sub4Arr2);
			   soulPoints = (byte)((randShort4 % 25) + 1);
		   }

		   var result = new Special(array, HexType.X1)
		   {
			   CustomType1 = HexType.X0, // Special is not stackable, no sp ratio
			   Quantity = 1,
		   };

		   result.Avatar.SoulPoints = soulPoints.Value;
		   return result;
		*/
		todo!()
	}
}
