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

//! Elements of governance concerning the Polkadot Fellowship. This is only a temporary arrangement
//! since the Polkadot Fellowship belongs under the Polkadot Relay. However, that is not yet in
//! place, so until then it will need to live here. Once it is in place and there exists a bridge
//! between Polkadot/Kusama then this code can be removed.

use frame_support::traits::{MapSuccess, TryMapSuccess};
use sp_arithmetic::traits::CheckedSub;
use sp_runtime::{
    morph_types,
    traits::{ConstU16, Replace, TypedGet},
};

use super::*;
use crate::DAYS;

parameter_types! {
    pub const AlarmInterval: BlockNumber = 1;
    pub const SubmissionDeposit: Balance = 0;
    pub const UndecidingTimeout: BlockNumber = 7 * DAYS;
}

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
    type Id = u16;
    type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;
    fn tracks() -> &'static [(Self::Id, pallet_referenda::TrackInfo<Balance, BlockNumber>)] {
        static DATA: [(u16, pallet_referenda::TrackInfo<Balance, BlockNumber>); 10] = [
            (
                0u16,
                pallet_referenda::TrackInfo {
                    name: "pre pissers",
                    max_deciding: 10,
                    decision_deposit: 100 * QUID,
                    prepare_period: 30 * MINUTES,
                    decision_period: 7 * DAYS,
                    confirm_period: 30 * MINUTES,
                    min_enactment_period: 1 * MINUTES,
                    min_approval: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(50),
                        ceil: Perbill::from_percent(100),
                    },
                    min_support: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(0),
                        ceil: Perbill::from_percent(50),
                    },
                },
            ),
            (
                1u16,
                pallet_referenda::TrackInfo {
                    name: "pissers",
                    max_deciding: 10,
                    decision_deposit: 10 * QUID,
                    prepare_period: 30 * MINUTES,
                    decision_period: 7 * DAYS,
                    confirm_period: 30 * MINUTES,
                    min_enactment_period: 1 * MINUTES,
                    min_approval: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(50),
                        ceil: Perbill::from_percent(100),
                    },
                    min_support: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(0),
                        ceil: Perbill::from_percent(50),
                    },
                },
            ),
            (
                2u16,
                pallet_referenda::TrackInfo {
                    name: "lads",
                    max_deciding: 10,
                    decision_deposit: 10 * QUID,
                    prepare_period: 30 * MINUTES,
                    decision_period: 7 * DAYS,
                    confirm_period: 30 * MINUTES,
                    min_enactment_period: 1 * MINUTES,
                    min_approval: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(50),
                        ceil: Perbill::from_percent(100),
                    },
                    min_support: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(0),
                        ceil: Perbill::from_percent(50),
                    },
                },
            ),
            (
                3u16,
                pallet_referenda::TrackInfo {
                    name: "big lads",
                    max_deciding: 10,
                    decision_deposit: 10 * QUID,
                    prepare_period: 30 * MINUTES,
                    decision_period: 7 * DAYS,
                    confirm_period: 30 * MINUTES,
                    min_enactment_period: 1 * MINUTES,
                    min_approval: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(50),
                        ceil: Perbill::from_percent(100),
                    },
                    min_support: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(0),
                        ceil: Perbill::from_percent(50),
                    },
                },
            ),
            (
                4u16,
                pallet_referenda::TrackInfo {
                    name: "massive legends",
                    max_deciding: 10,
                    decision_deposit: 10 * QUID,
                    prepare_period: 30 * MINUTES,
                    decision_period: 7 * DAYS,
                    confirm_period: 30 * MINUTES,
                    min_enactment_period: 1 * MINUTES,
                    min_approval: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(50),
                        ceil: Perbill::from_percent(100),
                    },
                    min_support: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(0),
                        ceil: Perbill::from_percent(50),
                    },
                },
            ),
            (
                5u16,
                pallet_referenda::TrackInfo {
                    name: "uber giga chads",
                    max_deciding: 10,
                    decision_deposit: 1 * QUID,
                    prepare_period: 30 * MINUTES,
                    decision_period: 7 * DAYS,
                    confirm_period: 30 * MINUTES,
                    min_enactment_period: 1 * MINUTES,
                    min_approval: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(50),
                        ceil: Perbill::from_percent(100),
                    },
                    min_support: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(0),
                        ceil: Perbill::from_percent(50),
                    },
                },
            ),
        ];
        &DATA[..]
    }
    fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
        use super::origins::Origin;

        #[cfg(feature = "runtime-benchmarks")]
        {
            // For benchmarks, we enable a root origin.
            // It is important that this is not available in production!
            let root: Self::RuntimeOrigin = frame_system::RawOrigin::Root.into();
            if &root == id {
                return Ok(9);
            }
        }

        match Origin::try_from(id.clone()) {
            Ok(Origin::ShipPrePissers) => Ok(0),
            Ok(Origin::ShipPisser) => Ok(1),
            Ok(Origin::ShipLad) => Ok(2),
            Ok(Origin::ShipBigLad) | Ok(Origin::ShipBigLadAndUp) => Ok(3),
            Ok(Origin::ShipMassiveLegend) | Ok(Origin::ShipMassiveLegendAndUp) => Ok(4),
            Ok(Origin::ShipUberGigaChad | Origin::ShipUberGigaChadAndUp) => Ok(5),
            _ => Err(()),
        }
    }
}
pallet_referenda::impl_tracksinfo_get!(TracksInfo, Balance, BlockNumber);

pub type ShipReferendaInstance = pallet_referenda::Instance2;

impl pallet_referenda::Config<ShipReferendaInstance> for Runtime {
    type WeightInfo = weights::pallet_referenda_fellowship_referenda::WeightInfo<Self>;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Scheduler = Scheduler;
    type Currency = Balances;
    type SubmitOrigin = pallet_ranked_collective::EnsureMember<Runtime, ShipCollectiveInstance, 1>;
    type CancelOrigin = ShipMassiveLegendAndUp;
    type KillOrigin = ShipMassiveLegendAndUp;
    type Slash = Treasury;
    type Votes = pallet_ranked_collective::Votes;
    type Tally = pallet_ranked_collective::TallyOf<Runtime, ShipCollectiveInstance>;
    type SubmissionDeposit = SubmissionDeposit;
    type MaxQueued = ConstU32<100>;
    type UndecidingTimeout = UndecidingTimeout;
    type AlarmInterval = AlarmInterval;
    type Tracks = TracksInfo;
    type Preimages = Preimage;
}

pub type ShipCollectiveInstance = pallet_ranked_collective::Instance1;

morph_types! {
    /// A `TryMorph` implementation to reduce a scalar by a particular amount, checking for
    /// underflow.
    pub type CheckedReduceBy<N: TypedGet>: TryMorph = |r: N::Type| -> Result<N::Type, ()> {
        r.checked_sub(&N::get()).ok_or(())
    } where N::Type: CheckedSub;
}

impl pallet_ranked_collective::Config<ShipCollectiveInstance> for Runtime {
    type WeightInfo = weights::pallet_ranked_collective::WeightInfo<Self>;
    type RuntimeEvent = RuntimeEvent;
    // Promotion is by any of:
    // - Root can demote arbitrarily.
    // - the FellowshipAdmin origin (i.e. token holder referendum);
    // - a vote by the rank *above* the new rank.
    type PromoteOrigin = EitherOf<
        EitherOf<
            frame_system::EnsureRootWithSuccess<Self::AccountId, ConstU16<65535>>,
            EitherOf<
                MapSuccess<FellowshipAdmin, Replace<ConstU16<9>>>,
                TryMapSuccess<origins::EnsureShip, CheckedReduceBy<ConstU16<1>>>,
            >,
        >,
    >;
    // Demotion is by any of:
    // - Root can demote arbitrarily.
    // - the FellowshipAdmin origin (i.e. token holder referendum);
    // - a vote by the rank two above the current rank.
    type DemoteOrigin = EitherOf<
        frame_system::EnsureRootWithSuccess<Self::AccountId, ConstU16<65535>>,
        EitherOf<
            MapSuccess<ShipAdmin, Replace<ConstU16<9>>>,
            TryMapSuccess<origins::EnsureShip, CheckedReduceBy<ConstU16<2>>>,
        >,
    >;
    type Polls = ShipReferenda;
    type MinRankOfClass = sp_runtime::traits::Identity;
    type VoteWeight = pallet_ranked_collective::Geometric;
}
