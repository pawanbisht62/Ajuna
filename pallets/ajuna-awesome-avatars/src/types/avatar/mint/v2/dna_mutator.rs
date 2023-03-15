use crate::{
	types::{avatar::types::*, Dna},
	Config,
};

pub(crate) trait DnaMutator<T: Config> {
	fn mutate_from_base(&self, base_dna: Dna) -> Dna;
}

impl<T> DnaMutator<T> for PetItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_dna: Dna) -> Dna {
		todo!()
	}
}

impl<T> DnaMutator<T> for MaterialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_dna: Dna) -> Dna {
		todo!()
	}
}

impl<T> DnaMutator<T> for EssenceItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_dna: Dna) -> Dna {
		todo!()
	}
}

impl<T> DnaMutator<T> for EquipableItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_dna: Dna) -> Dna {
		todo!()
	}
}

impl<T> DnaMutator<T> for BlueprintItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_dna: Dna) -> Dna {
		todo!()
	}
}

impl<T> DnaMutator<T> for SpecialItemType
where
	T: Config,
{
	fn mutate_from_base(&self, base_dna: Dna) -> Dna {
		todo!()
	}
}
