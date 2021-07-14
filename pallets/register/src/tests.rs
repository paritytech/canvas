use super::*;
use crate as pallet_compose_register;

use frame_support::{assert_ok, assert_noop, parameter_types};
use sp_core::{H256, Hasher};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		ComposeRegister: pallet_compose_register::{Pallet, Call, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = Call;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxDomainByteSize: u32 = 32;
	pub const MaxRouteByteSize: u32 = 512;
}
impl Config for Test {
	type Event = Event;
	type MaxDomainByteSize = MaxDomainByteSize;
	type MaxRouteByteSize = MaxRouteByteSize;
}

const ACCOUNT_A: AccountIdOf<Test> = 1;
const ACCOUNT_B: AccountIdOf<Test> = 2;
const ACCOUNT_C: AccountIdOf<Test> = 3;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let genesis = pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(ACCOUNT_A, 100),
			(ACCOUNT_B, 200),
			(ACCOUNT_C, 300),
		],
	};
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}

const DOMAIN_X: &str = "MyDomainX";
const DOMAIN_Y: &str = "MyDomainY";

#[test]
fn successful_domain_registration() {
    let mut chain = new_test_ext();

    chain.execute_with(|| {
		let domain = SerString::from(DOMAIN_X);
		let mangled_domain = SerString::from(DOMAIN_Y);
		let route = SerString::from("");
		let path = Path::new(domain.clone(), route.clone());
		let mangled_path = Path::new(mangled_domain.clone(), route.clone());
        let owner = ACCOUNT_A;
		let goal_result = RegisteredPath::new(&owner, &path, &None);
    
        // register the goal domain
        assert_ok!(ComposeRegister::register_domain(Origin::signed(owner), domain.clone()));

        // path was registered to A
		// if_std! { println!("in register_domain, Looking for the following K/K in RegisteredPaths = {:#?}\n   {:#?}", goal_result.path.domain.clone(), goal_result.path.route.clone()); }
        assert_eq!(ComposeRegister::get_registered_path_info(&path), Some(goal_result));
        assert_eq!(ComposeRegister::is_registered_path(&path), true);

        // mangled path was not registered to A
		// if_std! { println!("in register_domain, Looking for the following K/K in RegisteredPaths = {:#?}\n   {:#?}", goal_result.path.domain.clone(), goal_result.path.route.clone()); }
        assert_eq!(ComposeRegister::get_registered_path_info(&mangled_path), None);
        assert_eq!(ComposeRegister::is_registered_path(&mangled_path), false);

        // try to register the same domain a 2nd time
        assert_noop!(ComposeRegister::register_domain(
			Origin::signed(owner),
            domain.clone(),
        ), Error::<Test>::DomainAlreadyExists);

        // try to register B as the domain owner in place of A
        assert_noop!(ComposeRegister::register_domain(
			Origin::signed(ACCOUNT_B),
            domain.clone(),
        ), Error::<Test>::DomainAlreadyExists);
    });
}

#[test]
fn successful_approval_registration() {
    let mut chain = new_test_ext();

    chain.execute_with(|| {
		let domain = SerString::from(DOMAIN_X);
		let route = SerString::from("");
		let path = Path::new(domain.clone(), route.clone());
        let owner = ACCOUNT_A;

		// Register the domain to A
        assert_ok!(ComposeRegister::register_domain(Origin::signed(owner), domain.clone()));

        // Add B as approver on the domain
        assert_ok!(ComposeRegister::add_approver(Origin::signed(owner), domain.clone(), ACCOUNT_B));

        // Verify that A is an approver on the domain
        assert_eq!(ComposeRegister::is_registered_approver(&path, &ACCOUNT_A), true);
        // Verify that B is also an approver on the domain
        assert_eq!(ComposeRegister::is_registered_approver(&path, &ACCOUNT_B), true);
        // Verify that C is not an approver on the domain
        assert_eq!(ComposeRegister::is_registered_approver(&path, &ACCOUNT_C), false);
    });
}

#[test]
fn successful_contract_registration() {
    let mut chain = new_test_ext();

    chain.execute_with(|| {
		let domain = SerString::from(DOMAIN_X);
		let domain_lc = domain.to_lowercase();
		let route = SerString::from("a/route");
		let sub_route = SerString::from("a/route/to");
		let super_route = SerString::from("a");
		let path = Path::new(domain.clone(), route.clone());
		let path_lc = Path::new(domain_lc.clone(), route.clone());
		let sub_path = Path::new(domain.clone(), sub_route.clone());
		// let sub_path_lc = Path::new(domain_lc.clone(), sub_route.clone());
		let super_path = Path::new(domain.clone(), super_route.clone());
		let super_path_lc = Path::new(domain_lc.clone(), super_route.clone());
        let owner = ACCOUNT_A;
		let code = vec![1, 2, 3];
		let hash = <<Test as frame_system::Config>::Hashing as Hasher>::hash(&code);
		let contract = Contract::<Test>::new(&hash, &Some(code.clone()));
		let registered_path = RegisteredPath { 
			owner, 
			path: path_lc.clone(), 
			contract: Some(contract) 
		};
		let registered_super_path = RegisteredPath { 
			owner, 
			path: super_path_lc.clone(), 
			contract: None 
		};

		// Register the domain to A
        assert_ok!(ComposeRegister::register_domain(Origin::signed(owner), domain.clone()));
		// Register the contract to a path in the domain owned by A
        assert_ok!(ComposeRegister::register_contract(Origin::signed(owner), path.path_lc.clone(), hash.clone(), Some(code.clone())));

        assert_eq!(ComposeRegister::is_registered_path(&path), true);
        assert_eq!(ComposeRegister::is_registered_path(&sub_path), false);
        assert_eq!(ComposeRegister::is_registered_path(&super_path), true);

        assert_eq!(ComposeRegister::get_registered_path_info(&path), Some(registered_path));
        assert_eq!(ComposeRegister::get_registered_path_info(&sub_path), None);
		assert_eq!(ComposeRegister::get_registered_path_info(&super_path), Some(registered_super_path));
    });
}

