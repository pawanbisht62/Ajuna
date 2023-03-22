use super::*;

pub(crate) const MAX_QUANTITY: u8 = 8;

// This value comes from the following calculation -> 256 * 0.05 -> 12.8 -> 13
pub(crate) const STACK_PROBABILITY: u8 = 13;

/// Probabilities for all PackType::Material options
pub(crate) const PACK_TYPE_MATERIAL_ITEM_PROBABILITIES: ProbabilitySlots<ItemType, 6> = [
	(ItemType::Pet, 150),
	(ItemType::Material, 700),
	(ItemType::Essence, 50),
	(ItemType::Equipable, 100),
	(ItemType::Blueprint, 0),
	(ItemType::Special, 0),
];

pub(crate) const PACK_TYPE_MATERIAL_PET_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<PetItemType, 3> =
	[(PetItemType::Pet, 0), (PetItemType::PetPart, 980), (PetItemType::Egg, 20)];

pub(crate) const PACK_TYPE_MATERIAL_MATERIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	MaterialItemType,
	8,
> = [
	(MaterialItemType::Polymers, 125),
	(MaterialItemType::Electronics, 125),
	(MaterialItemType::PowerCells, 125),
	(MaterialItemType::Optics, 125),
	(MaterialItemType::Metals, 125),
	(MaterialItemType::Ceramics, 125),
	(MaterialItemType::Superconductors, 125),
	(MaterialItemType::Nanomaterials, 125),
];

pub(crate) const PACK_TYPE_MATERIAL_ESSENCE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EssenceItemType,
	3,
> = [
	(EssenceItemType::Glimmer, 400),
	(EssenceItemType::ColorSpark, 350),
	(EssenceItemType::GlowSpark, 250),
];

pub(crate) const PACK_TYPE_MATERIAL_EQUIPABLE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EquipableItemType,
	7,
> = [
	(EquipableItemType::ArmorBase, 820),
	(EquipableItemType::ArmorComponent1, 50),
	(EquipableItemType::ArmorComponent2, 50),
	(EquipableItemType::ArmorComponent3, 50),
	(EquipableItemType::WeaponVersion1, 10),
	(EquipableItemType::WeaponVersion2, 10),
	(EquipableItemType::WeaponVersion3, 10),
];

pub(crate) const PACK_TYPE_MATERIAL_BLUEPRINT_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	BlueprintItemType,
	1,
> = [(BlueprintItemType::Blueprint, 1000)];

pub(crate) const PACK_TYPE_MATERIAL_SPECIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	SpecialItemType,
	1,
> = [(SpecialItemType::Special, 1000)];
// -----------------------------------------------

/// Probabilities for all PackType::Equipment options
pub(crate) const PACK_TYPE_EQUIPMENT_ITEM_PROBABILITIES: ProbabilitySlots<ItemType, 6> = [
	(ItemType::Pet, 90),
	(ItemType::Material, 200),
	(ItemType::Essence, 10),
	(ItemType::Equipable, 700),
	(ItemType::Blueprint, 0),
	(ItemType::Special, 0),
];

pub(crate) const PACK_TYPE_EQUIPMENT_PET_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<PetItemType, 3> =
	[(PetItemType::Pet, 0), (PetItemType::PetPart, 800), (PetItemType::Egg, 200)];

pub(crate) const PACK_TYPE_EQUIPMENT_MATERIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	MaterialItemType,
	8,
> = [
	(MaterialItemType::Polymers, 125),
	(MaterialItemType::Electronics, 125),
	(MaterialItemType::PowerCells, 125),
	(MaterialItemType::Optics, 125),
	(MaterialItemType::Metals, 125),
	(MaterialItemType::Ceramics, 125),
	(MaterialItemType::Superconductors, 125),
	(MaterialItemType::Nanomaterials, 125),
];

pub(crate) const PACK_TYPE_EQUIPMENT_ESSENCE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EssenceItemType,
	3,
> = [
	(EssenceItemType::Glimmer, 400),
	(EssenceItemType::ColorSpark, 350),
	(EssenceItemType::GlowSpark, 250),
];

pub(crate) const PACK_TYPE_EQUIPMENT_EQUIPABLE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EquipableItemType,
	7,
> = [
	(EquipableItemType::ArmorBase, 820),
	(EquipableItemType::ArmorComponent1, 50),
	(EquipableItemType::ArmorComponent2, 50),
	(EquipableItemType::ArmorComponent3, 50),
	(EquipableItemType::WeaponVersion1, 10),
	(EquipableItemType::WeaponVersion2, 10),
	(EquipableItemType::WeaponVersion3, 10),
];

pub(crate) const PACK_TYPE_EQUIPMENT_BLUEPRINT_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	BlueprintItemType,
	1,
> = [(BlueprintItemType::Blueprint, 1000)];

pub(crate) const PACK_TYPE_EQUIPMENT_SPECIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	SpecialItemType,
	1,
> = [(SpecialItemType::Special, 1000)];
// -----------------------------------------------

/// Probabilities for all PackType::Special options
pub(crate) const PACK_TYPE_SPECIAL_ITEM_PROBABILITIES: ProbabilitySlots<ItemType, 6> = [
	(ItemType::Pet, 100),
	(ItemType::Material, 150),
	(ItemType::Essence, 50),
	(ItemType::Equipable, 700),
	(ItemType::Blueprint, 0),
	(ItemType::Special, 0),
];

pub(crate) const PACK_TYPE_SPECIAL_PET_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<PetItemType, 3> =
	[(PetItemType::Pet, 0), (PetItemType::PetPart, 0), (PetItemType::Egg, 1000)];

pub(crate) const PACK_TYPE_SPECIAL_MATERIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	MaterialItemType,
	8,
> = [
	(MaterialItemType::Polymers, 125),
	(MaterialItemType::Electronics, 125),
	(MaterialItemType::PowerCells, 125),
	(MaterialItemType::Optics, 125),
	(MaterialItemType::Metals, 125),
	(MaterialItemType::Ceramics, 125),
	(MaterialItemType::Superconductors, 125),
	(MaterialItemType::Nanomaterials, 125),
];

pub(crate) const PACK_TYPE_SPECIAL_ESSENCE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EssenceItemType,
	3,
> = [
	(EssenceItemType::Glimmer, 400),
	(EssenceItemType::ColorSpark, 350),
	(EssenceItemType::GlowSpark, 250),
];

pub(crate) const PACK_TYPE_SPECIAL_EQUIPABLE_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	EquipableItemType,
	7,
> = [
	(EquipableItemType::ArmorBase, 250),
	(EquipableItemType::ArmorComponent1, 200),
	(EquipableItemType::ArmorComponent2, 200),
	(EquipableItemType::ArmorComponent3, 200),
	(EquipableItemType::WeaponVersion1, 50),
	(EquipableItemType::WeaponVersion2, 50),
	(EquipableItemType::WeaponVersion3, 50),
];

pub(crate) const PACK_TYPE_SPECIAL_BLUEPRINT_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	BlueprintItemType,
	1,
> = [(BlueprintItemType::Blueprint, 1000)];

pub(crate) const PACK_TYPE_SPECIAL_SPECIAL_ITEM_TYPE_PROBABILITIES: ProbabilitySlots<
	SpecialItemType,
	1,
> = [(SpecialItemType::Special, 1000)];
// -----------------------------------------------

/// Probabilities for equipment slots
pub(crate) const ARMOR_SLOT_PROBABILITIES: ProbabilitySlots<SlotType, 6> = [
	(SlotType::Head, 170),
	(SlotType::Breast, 170),
	(SlotType::ArmFront, 165),
	(SlotType::ArmBack, 165),
	(SlotType::LegFront, 165),
	(SlotType::LegBack, 165),
];

pub(crate) const WEAPON_SLOT_PROBABILITIES: ProbabilitySlots<SlotType, 2> =
	[(SlotType::WeaponFront, 500), (SlotType::WeaponBack, 500)];

/// Probabilities for pet type
pub(crate) const PET_TYPE_PROBABILITIES: ProbabilitySlots<PetType, 7> = [
	(PetType::TankyBulldog, 150),
	(PetType::FoxishDude, 150),
	(PetType::WierdFerry, 150),
	(PetType::FireDino, 150),
	(PetType::BigHybrid, 150),
	(PetType::GiantWoodStick, 150),
	(PetType::CrazyDude, 100),
];