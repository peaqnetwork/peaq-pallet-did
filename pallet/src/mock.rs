use crate as peaq_did;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub(crate) type Balance = u128;
pub(crate) type AccountId = sr25519::Public;
pub(crate) type Moment = u64;

pub(crate) const EXISTENTIAL_DEPOSIT: Balance = 2;
pub(crate) const DEPOSIT_BASE: Balance = 100;
pub(crate) const DEPOSIT_PER_BYTE: Balance = 2;
pub(crate) const BOUNDED_DATA_LEN: u32 = 2560;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        PeaqDID: peaq_did::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
    pub const BlockNumber: u64 = 5;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
    pub const BoundedDataLen: u32 = BOUNDED_DATA_LEN;
}

impl pallet_timestamp::Config for Test {
    type Moment = Moment;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const MaxLocks: u32 = 4;
    pub const MaxReserves: u32 = 4;
    pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type HoldIdentifier = ();
    type MaxFreezes = ();
}

parameter_types! {
    pub const StorageDepositBase: Balance = DEPOSIT_BASE;
    pub const StorageDepositPerByte: Balance = DEPOSIT_PER_BYTE;
    pub const StorageReserveIdentifier: [u8; 8] = [b'p', b'e', b'a', b'q', b'_', b'd', b'i', b'd'];
}

impl peaq_did::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Time = pallet_timestamp::Pallet<Test>;
    type WeightInfo = peaq_did::weights::WeightInfo<Test>;
    type BoundedDataLen = BoundedDataLen;
    type StorageDepositBase = StorageDepositBase;
    type StorageDepositPerByte = StorageDepositPerByte;
    type Currency = Balances;
    type ReserveIdentifier = StorageReserveIdentifier;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let (acc1, acc2, acc3, _, _) = test_accounts();
    let mut storage = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    // This will cause some initial issuance
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (acc1, 1400000000000000000000000000),
            (acc2, 1400000000000000000000000000),
            (acc3, 1400000000000000000000000000),
        ],
    }
    .assimilate_storage(&mut storage)
    .ok();

    let mut ext = sp_io::TestExternalities::from(storage);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

// First 3 accounts are funded, the rest are not.
pub fn test_accounts() -> (AccountId, AccountId, AccountId, AccountId, AccountId) {
    (
        account_key("Iredia"),
        account_key("Iredia1"),
        account_key("Iredia2"),
        account_key("Talha"),
        account_key("Talha1"),
    )
}

pub fn account_key(s: &str) -> sr25519::Public {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}
