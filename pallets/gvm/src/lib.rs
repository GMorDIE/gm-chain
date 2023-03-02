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
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::error]
    pub enum Error<T> {
        ExecutionFailed,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        ExecutionCompleted {
            original_mem: Vec<u8>,
            new_mem: Vec<u8>,
            error: bool,
        },
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transfer some balance to another account under `currency_id`.
        ///
        /// The dispatch origin for this call must be `Signed` by the
        /// transactor.
        #[pallet::weight(1000000000)]
        pub fn run(origin: OriginFor<T>, contract: Vec<u8>) -> DispatchResult {
            match interpret(contract.clone()) {
                Ok(new_mem) => Self::deposit_event(Event::<T>::ExecutionCompleted {
                    original_mem: contract,
                    new_mem,
                    error: false,
                }),

                Err(new_mem) => Self::deposit_event(Event::<T>::ExecutionCompleted {
                    original_mem: contract,
                    new_mem,
                    error: true,
                }),
            }

            Ok(())
        }
    }
}

pub(crate) fn interpret(contract: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
    let mut new_mem: Vec<u8> = contract.clone();

    let mut ip: u8 = 0;

    loop {
        let f: u8 = new_mem[(ip / 8) as usize];
        let j: u8 = new_mem[(ip / 8 + 1) as usize];

        if ip % 8 != 0 {
            return Err(new_mem);
        }

        if f >= ip && f < ip + 16 {
            return Err(new_mem);
        }

        if ip == j {
            return Ok(new_mem);
        }

        new_mem[(f / 8) as usize] ^= 1 << f % 8;
        ip = j;
    }
}
