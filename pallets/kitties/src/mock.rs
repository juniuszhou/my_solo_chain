use crate as pallet_kitties;
use polkadot_sdk::frame_support::traits::Hooks;
use polkadot_sdk::frame_support::{
    derive_impl,
    traits::{ConstU16, ConstU64},
};
use polkadot_sdk::sp_core::H256;
use polkadot_sdk::sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
polkadot_sdk::frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Kitties: pallet_kitties,
        Random: pallet_insecure_randomness_collective_flip,
    }
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_kitties::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Randomness = Random;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .try_init();
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        Kitties::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Kitties::on_initialize(System::block_number());
    }
}
