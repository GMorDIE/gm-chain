use crate::{
    AccountId, Balance, Balances, BlockNumber, Call, Event, Origin, OriginCaller, Preimage,
    Runtime, RuntimeBlockWeights, Scheduler, Treasury, Weight, DAYS, HOURS, MILLIUNIT, MINUTES,
    UNIT,
};
use core::ops::Mul;
use frame_support::{
    parameter_types,
    traits::{EitherOfDiverse, EqualPrivilegeOnly},
};
use frame_system::EnsureRoot;
use pallet_collective::{self, EnsureProportionAtLeast};
use sp_runtime::Perbill;

pub type MajorityCouncilOrRoot = EitherOfDiverse<
    EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
    EnsureRoot<AccountId>,
>;
pub type UnanimousCouncilOrRoot = EitherOfDiverse<
    EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>,
    EnsureRoot<AccountId>,
>;
pub type SuperMajorityCouncilOrRoot = EitherOfDiverse<
    EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>,
    EnsureRoot<AccountId>,
>;

parameter_types! {
    pub const LaunchPeriod: BlockNumber = 3 * DAYS;
    pub const VotingPeriod: BlockNumber = 3 * DAYS;
    pub const FastTrackVotingPeriod: BlockNumber = 3 * HOURS;
    pub const MinimumDeposit: Balance = 1000 * UNIT;
    pub const EnactmentPeriod: BlockNumber = 6 * DAYS;
    pub const CooloffPeriod: BlockNumber = 7 * DAYS;
    pub const PreimageByteDeposit: Balance = 10 * MILLIUNIT;
    pub const InstantAllowed: bool = true;
    pub const MaxVotes: u32 = 1000;
    pub const MaxProposals: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
    type Proposal = Call;
    type Event = Event;
    type Currency = Balances;
    type EnactmentPeriod = EnactmentPeriod;
    type LaunchPeriod = LaunchPeriod;
    type VotingPeriod = VotingPeriod;
    type MinimumDeposit = MinimumDeposit;
    /// A straight majority of the council can decide what their next motion is.
    type ExternalOrigin = MajorityCouncilOrRoot;
    /// A majority can have the next scheduled referendum be a straight majority-carries vote
    type ExternalMajorityOrigin = MajorityCouncilOrRoot;
    /// A unanimous council can have the next scheduled referendum be a straight default-carries
    /// (NTB) vote.
    type ExternalDefaultOrigin = UnanimousCouncilOrRoot;
    /// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
    /// be tabled immediately and with a shorter voting/enactment period.
    type FastTrackOrigin = MajorityCouncilOrRoot;
    type InstantOrigin = UnanimousCouncilOrRoot;
    type InstantAllowed = InstantAllowed;
    type FastTrackVotingPeriod = FastTrackVotingPeriod;
    // To cancel a proposal which has been passed, 2/3 of the council must agree to it.
    type CancellationOrigin = SuperMajorityCouncilOrRoot;
    // To cancel a proposal before it has been passed, the technical committee must be unanimous or
    // Root must agree.
    type CancelProposalOrigin = UnanimousCouncilOrRoot;
    type BlacklistOrigin = EnsureRoot<AccountId>;
    // Any single technical committee member may veto a coming council proposal, however they can
    // only do it once and it lasts only for the cooloff period.
    type VetoOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
    type CooloffPeriod = CooloffPeriod;
    type PreimageByteDeposit = PreimageByteDeposit;
    type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
    type Slash = Treasury;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type MaxVotes = MaxVotes;
    type WeightInfo = ();
    type MaxProposals = MaxProposals;
    type VoteLockingPeriod = EnactmentPeriod;
}

parameter_types! {
    pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
    pub const CouncilMaxProposals: u32 = 100;
    pub const CouncilMaxMembers: u32 = 50;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = CouncilMotionDuration;
    type MaxProposals = CouncilMaxProposals;
    type MaxMembers = CouncilMaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
    type WeightInfo = ();
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80).mul(RuntimeBlockWeights::get().max_block);
    pub const MaxScheduledPerBlock: u32 = 10;
    pub const NoPreimagePostponement: Option<u32> = Some(5 * MINUTES);
}

impl pallet_scheduler::Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = ();
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type PreimageProvider = Preimage;
    type NoPreimagePostponement = NoPreimagePostponement;
}

parameter_types! {
    pub const PreimageMaxSize: u32 = 4096 * 1024;
    pub PreimageBaseDeposit: Balance = 10 * MILLIUNIT;
}

impl pallet_preimage::Config for Runtime {
    type WeightInfo = ();
    type Event = Event;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type MaxSize = PreimageMaxSize;
    type BaseDeposit = PreimageBaseDeposit;
    type ByteDeposit = PreimageByteDeposit;
}
