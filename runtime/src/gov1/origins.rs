// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot. If not, see <http://www.gnu.org/licenses/>.

//! Custom origins for governance interventions.

pub use pallet_custom_origins::*;

#[frame_support::pallet]
pub mod pallet_custom_origins {
    use crate::{Balance, UNIT};
    use frame_support::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[derive(PartialEq, Eq, Clone, MaxEncodedLen, Encode, Decode, TypeInfo, RuntimeDebug)]
    #[pallet::origin]
    pub enum Origin {
        /// Origin for spending (any amount of) funds.
        Baggooooor,
        /// Origin for managing the composition of the fellowship.
        ShipAdmin,
        /// Origin for managing the registrar.
        GeneralAdmin,
        /// Origin able to cancel referenda.
        ReferendumCanceller,
        /// Origin able to kill referenda.
        ReferendumKiller,
        /// Origin able to spend up to 1 KSM from the treasury at once.
        LilGrifter,
        /// Origin able to spend up to 5 KSM from the treasury at once.
        YugeGrifter,
        /// Origin able to dispatch a whitelisted call.
        WhitelistedCaller,
        /// Origin commanded by any members of the Polkadot Fellowship (no Dan grade needed).
        ShipPrePissers,
        /// Origin commanded by Polkadot Fellows (3rd Dan fellows or greater).
        ShipBigLadAndUp,
        /// Origin commanded by Polkadot Experts (5th Dan fellows or greater).
        ShipMassiveLegendAndUp,
        /// Origin commanded by Polkadot Masters (7th Dan fellows of greater).
        ShipUberGigaChadAndUp,
        /// Origin commanded by rank 1 of the Polkadot Fellowship and with a success of 1.
        ShipPisser,
        /// Origin commanded by rank 2 of the Polkadot Fellowship and with a success of 2.
        ShipLad,
        /// Origin commanded by rank 3 of the Polkadot Fellowship and with a success of 3.
        ShipBigLad,
        /// Origin commanded by rank 4 of the Polkadot Fellowship and with a success of 4.
        ShipMassiveLegend,
        /// Origin commanded by rank 5 of the Polkadot Fellowship and with a success of 5.
        ShipUberGigaChad,
    }

    macro_rules! decl_unit_ensures {
		( $name:ident: $success_type:ty = $success:expr ) => {
			pub struct $name;
			impl<O: Into<Result<Origin, O>> + From<Origin>>
				EnsureOrigin<O> for $name
			{
				type Success = $success_type;
				fn try_origin(o: O) -> Result<Self::Success, O> {
					o.into().and_then(|o| match o {
						Origin::$name => Ok($success),
						r => Err(O::from(r)),
					})
				}
				#[cfg(feature = "runtime-benchmarks")]
				fn try_successful_origin() -> Result<O, ()> {
					Ok(O::from(Origin::$name))
				}
			}
		};
		( $name:ident ) => { decl_unit_ensures! { $name : () = () } };
		( $name:ident: $success_type:ty = $success:expr, $( $rest:tt )* ) => {
			decl_unit_ensures! { $name: $success_type = $success }
			decl_unit_ensures! { $( $rest )* }
		};
		( $name:ident, $( $rest:tt )* ) => {
			decl_unit_ensures! { $name }
			decl_unit_ensures! { $( $rest )* }
		};
		() => {}
	}
    decl_unit_ensures!(
        StakingAdmin,
        Baggooooor,
        ShipAdmin,
        GeneralAdmin,
        ReferendumCanceller,
        ReferendumKiller,
        WhitelistedCaller,
        ShipPrePissers: u16 = 0,
        ShipBigLadAndUp: u16 = 3,
        ShipMassiveLegendAndUp: u16 = 5,
        ShipUberGigaChadAndUp: u16 = 7,
    );

    macro_rules! decl_ensure {
		(
			$vis:vis type $name:ident: EnsureOrigin<Success = $success_type:ty> {
				$( $item:ident = $success:expr, )*
			}
		) => {
			$vis struct $name;
			impl<O: Into<Result<Origin, O>> + From<Origin>>
				EnsureOrigin<O> for $name
			{
				type Success = $success_type;
				fn try_origin(o: O) -> Result<Self::Success, O> {
					o.into().and_then(|o| match o {
						$(
							Origin::$item => Ok($success),
						)*
						r => Err(O::from(r)),
					})
				}
				#[cfg(feature = "runtime-benchmarks")]
				fn try_successful_origin() -> Result<O, ()> {
					// By convention the more privileged origins go later, so for greatest chance
					// of success, we want the last one.
					let _result: Result<O, ()> = Err(());
					$(
						let _result: Result<O, ()> = Ok(O::from(Origin::$item));
					)*
					_result
				}
			}
		}
	}

    decl_ensure! {
        pub type Spender: EnsureOrigin<Success = Balance> {
            LilGrifter = 10_000 * UNIT,
            YugeGrifter = 100_000 * UNIT,
            Baggooooor = 69_420_000 * UNIT,
        }
    }

    decl_ensure! {
        pub type EnsureShip: EnsureOrigin<Success = u16> {
            ShipPisser = 1,
            ShipLad = 2,
            ShipBigLad = 3,
            ShipMassiveLegend = 4,
            ShipUberGigaChad = 5,
        }
    }
}
