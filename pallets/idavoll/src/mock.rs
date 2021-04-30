
// use crate::{Module, Trait,OrgInfoOf,ProposalOf,OrgInfo,OrgRuleParam,
// 			Proposal,ProposalDetail};
use super::*;
use crate as idavoll;
use frame_support::{impl_outer_origin,impl_outer_dispatch, assert_ok, assert_noop, parameter_types, weights::Weight};
use sp_core::H256;
use sp_runtime::{Perbill, traits::{BlakeTwo256, IdentityLookup,Hash}, testing::Header,ModuleId};
use pallet_balances;
use frame_system::RawOrigin;
use sp_std::{prelude::Vec, boxed::Box,collections::btree_map::BTreeMap};


impl_outer_origin! {
		pub enum Origin for Test where system = frame_system {}
	}
impl_outer_dispatch! {
		pub enum Call for Test where origin: Origin {
			frame_system::System,
			pallet_balances::IdvBalances,
			idavoll::IdavollModule,
        }
    }

pub type System = frame_system::Module<Test>;
pub type IdvBalances = pallet_balances::Module<Test>;
pub type IdavollAsset = idavoll_asset::Module<Test>;
pub type IdavollAssetError = idavoll_asset::Error<Test>;

pub const A: u128 = 100;
pub const B: u128 = 200;
pub const OWNER: u128 = 88;
pub const RECEIVER: u128 = 7;
pub const ORGID: u128 = 1000;
pub const ORGID2: u128 = 2000;

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::one();
		pub const IdvAssetModuleId: ModuleId = ModuleId(*b"py/asset");
		pub const IdavollModuleId: ModuleId = ModuleId(*b"py/idvol");
	}
impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Index = u64;
	type Call = Call;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u128;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type AvailableBlockRatio = AvailableBlockRatio;
	type MaximumBlockLength = MaximumBlockLength;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 10;
    }
impl pallet_balances::Trait for Test {
	type Balance = u64;
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxLocks = ();
	type WeightInfo = ();
}

impl idavoll_asset::Trait for Test {
	type Event = ();
	type Balance = u64;
	type AssetId = u32;
	type Currency = IdvBalances;
	type ModuleId = IdvAssetModuleId;
}

pub type IdavollModule = Module<Test>;
type IdavallCall = idavoll::Call<Test>;
impl Trait for Test {
	type Event = ();
	type Call = Call;
	type Balance = u64;
	type AssetId = u32;
	type ModuleId = IdavollModuleId;
	type AssetHandle = IdavollAsset;
	type Finance = IdavollAsset;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let genesis = pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(A, 100000),
			(B, 200000),
		],
	};
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}
pub fn set_block_number(n: <Test as frame_system::Trait>::BlockNumber) -> <Test as frame_system::Trait>::BlockNumber {
	System::set_block_number(n);
	n
}
pub fn get_block_number() -> <Test as frame_system::Trait>::BlockNumber {
	System::block_number()
}
pub fn make_transfer_fail_proposal(value: u64) -> Vec<u8> {
	Call::IdvBalances(pallet_balances::Call::transfer(RECEIVER.clone(), value)).encode()
}
pub fn make_transfer_proposal(value: u64) -> Vec<u8> {
	Call::IdavollModule(IdavallCall::transfer(RECEIVER.clone(),value)).encode()
}
pub fn make_system_proposal(value: u64) -> Vec<u8> {
	Call::System(frame_system::Call::remark(vec![0; 1])).encode()
}

pub fn create_org(creator: u128) -> OrgInfoOf<Test> {
	let mut org = OrgInfo::new();
	org.members = vec![creator,1,2,3];
	org.param = OrgRuleParam::new(60,5,0);
	org.clone()
}
pub fn create_proposal(id: u128,call: Vec<u8>) -> ProposalOf<Test> {
	let sub_param = OrgRuleParam::new(60,5,0);
	Proposal {
		org:    id.clone(),
		call: 	call.clone(),
		detail: ProposalDetail::new(OWNER.clone(),5,sub_param.clone()),
	}
}

pub fn create_new_organization(creator: u128,total: u64) -> u128 {
	let info = create_org(creator);
	match IdavollModule::create_origanization(RawOrigin::Signed(creator).into(),total,info) {
		Ok(val) => {
			let c = OrgCounter::get();
			IdavollModule::counter2Orgid(c)
		},
		Err(e) => u128::MAX,
	}
}