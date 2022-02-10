// Creating mock runtime here

use crate as pallet_simple_nft;
use codec::{Decode, Encode};
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
use vitalam_pallet_traits::{ProcessFullyQualifiedId, ProcessIO, ProcessValidator};

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        SimpleNFTModule: pallet_simple_nft::{Module, Call, Storage, Event<T>},
    }
);
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
}

parameter_types! {
    pub const MaxMetadataCount: u32 = 4;
}

#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub enum Role {
    Owner,
    NotOwner,
}

impl Default for Role {
    fn default() -> Self {
        Role::Owner
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq)]
pub enum MetadataValue<TokenId> {
    File(Hash),
    Literal([u8; 1]),
    TokenId(TokenId),
    None,
}

impl<T> Default for MetadataValue<T> {
    fn default() -> Self {
        MetadataValue::None
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Debug, Eq)]
pub enum ProcessIdentifier {
    ShouldSucceed,
    ShouldFail,
}

impl Default for ProcessIdentifier {
    fn default() -> Self {
        ProcessIdentifier::ShouldSucceed
    }
}

pub struct MockProcessValidator {}

impl ProcessValidator<u64, Role, u64, MetadataValue<u64>> for MockProcessValidator {
    type ProcessIdentifier = ProcessIdentifier;
    type ProcessVersion = u32;

    fn validate_process(
        id: ProcessFullyQualifiedId<ProcessIdentifier, u32>,
        _sender: &u64,
        _inputs: &Vec<ProcessIO<u64, Role, u64, MetadataValue<u64>>>,
        _outputs: &Vec<ProcessIO<u64, Role, u64, MetadataValue<u64>>>,
    ) -> bool {
        id.id == ProcessIdentifier::ShouldSucceed
    }
}

impl pallet_simple_nft::Config for Test {
    type Event = Event;

    type TokenId = u64;
    type RoleKey = Role;
    type TokenMetadataKey = u64;
    type TokenMetadataValue = MetadataValue<Self::TokenId>;

    type ProcessValidator = MockProcessValidator;
    type WeightInfo = ();

    type MaxMetadataCount = MaxMetadataCount;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}