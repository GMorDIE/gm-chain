#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use core::marker::PhantomData;
use frame_support::pallet_prelude::StorageValue;
use frame_support::pallet_prelude::ValueQuery;
use frame_support::traits::ChangeMembers;
use sp_std::vec::Vec;

pub use module::*;

#[frame_support::pallet]
pub mod module {
    use super::*;

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {}

    #[pallet::error]
    pub enum Error<T, I = ()> {}

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

    #[pallet::storage]
    #[pallet::getter(fn members)]
    pub type Members<T: Config<I>, I: 'static = ()> =
        StorageValue<_, Vec<T::AccountId>, ValueQuery>;
}

impl<T: Config<I>, I: 'static> ChangeMembers<<T as frame_system::Config>::AccountId>
    for Pallet<T, I>
{
    fn change_members_sorted(
        _incoming: &[T::AccountId],
        _outgoing: &[T::AccountId],
        new: &[T::AccountId],
    ) {
        Members::<T, I>::put(new);
    }
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
    pub fn put_members(new: &[T::AccountId]) {
        Members::<T, I>::put(new);
    }
}
