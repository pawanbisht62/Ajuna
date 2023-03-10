use crate::*;
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
		todo!()
	}
}
