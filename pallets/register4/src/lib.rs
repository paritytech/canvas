#![cfg_attr(not(feature = "std"), no_std)]
// #![cfg_attr(feature = "runtime-benchmarks", recursion_limit="512")]

#[cfg(test)]
mod tests;

// use std::{
// 	marker::PhantomData,
// };

use sp_std::prelude::*;
use sp_std::if_std;
use frame_support::{
	RuntimeDebugNoBound,
};
use codec::{Encode, Decode};

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

type Domain = String;

type Route = String;

// pub enum PathGen<'a> {
//     /// A value domain/route combination, with domain D separate from route R.
//     DomainRoute(&'a String, &'a String),
//     /// A path, expressed as <domain> (if route is None) or <domain>:<route>.
//     Path(&'a String),
// }

/// The hash value of the WASM code in the smart contract.
type CodeHashOf<T> = <T as frame_system::Config>::Hash;

/// The WASM code instantiated in the smart contract.
type Code = Vec<u8>;

/// An approver is an account that has been approved by the path owner to also manage the path.  The approver's management 
/// rights are fully the same as the path owner's rights within the path's sub-domain.  E.g., if an approver is
/// given approval rights at the domain level (such as for "MyAPI"), then the approver may execute any
/// domain management calls.  If an approver is given approval rights for a given path (such as for "MyAPI:delegator"),
/// the approver may for example add or remove sub-paths (such as "MyAPI:delegator/v2.0", "MyAPI:delegator/LTS")
type Approver<T> = AccountIdOf<T>;


/// A path is a case-insensitive <domain>[:<route>/[<sub-route>/[...]]] UTF-8 string.  Examples of valid paths are:
///          MyAPI:delegator
///          myapi:adder
///          myapi:subber
///          MariasBakery:Order/Cookies/v2.1
///          mariasbakery:order/cakes/v2.0
///    		 MariasBakery                       <-- this is a pure / top-level domain
#[derive(Clone, Eq, PartialEq, RuntimeDebugNoBound, Encode, Decode)]
pub struct Path {
	/// <domain> as a string with original case preserved. If present any ":" will be removed
	/// (e.g., the domain is "MariasBakery" rather than "MariasBakery:")
	pub domain: Domain,

	/// <domain> as a lower-cased string.
	pub domain_lc: Domain,

	/// <route> as a string with original case preserved.  
	/// A top-level domain has a route of "".
	/// Routes are also UTF-8 strings, where sub-routes within the route are slash-separated.
	/// Routes are case-insensitive; Order/Cookies and order/cookies are considered the same route.
	/// A route of None corresponds to a pure / top-level (i.e., a route-less) domain
	pub route: Route,

	/// <route> as a lower-cased string.
	pub route_lc: Domain,

	/// <domain>: if route is none, else <domain>:<route>, as a string with original case preserved.
	pub path: String,

	/// <path> as a lower-cased string.
	pub path_lc: String,
}


impl Path {
	pub fn new(domain: &String, route: &String) -> Result<Path, String> {
		let domain = domain.clone().replace(":", "");
		let domain_lc: String = domain.clone().to_lowercase();
		let route = route.clone();
		let route_lc: String = route.clone().to_lowercase();
		let path: String = format!("{}:{}", domain.clone(), route);
		let path_lc: String = path.clone().to_lowercase();

		Ok(Path {
			domain,
			domain_lc,
			route,
			route_lc,
			path,
			path_lc,
		})
	}

	pub fn new_from_path_string(path_string: &String) -> Result<Path, String> {
		let split_path: Vec<&str> = path_string.split(":").collect();
		if (split_path.len() == 1) || (split_path.len() == 2) {
			let route: String = if split_path.len() == 1 {
				String::from("")
			} else {
				String::from(split_path[1].clone())
			};
			Self::new(&String::from(split_path[0].clone()), &route)
		} else {
			Err(String::from("Malformed Path"))
		}
	}
}


/// The code hash for a smart contract, and optionally the underlying code of that contract
#[derive(Clone, Eq, PartialEq, RuntimeDebugNoBound, Encode, Decode)]
pub struct Contract<T: frame_system::Config> {
	/// Optional: hash for the code referenced by this contract.
	pub code_hash: CodeHashOf<T>,

	/// Optional: code for this contract.
	pub code: Option<Code>,
}


impl<T: frame_system::Config> Contract<T> {
	pub fn new(code_hash: &CodeHashOf<T>, code: &Option<Code>) -> Self {
		Self {
			code_hash: code_hash.clone(),
			code: code.clone()
		}
	}
}


/// Registered path.
#[derive(Clone, Eq, PartialEq, RuntimeDebugNoBound, Encode, Decode)]
pub struct RegisteredPath<T: frame_system::Config> {
	/// Owner of the path.
	pub owner: AccountIdOf<T>,

	/// Full path (i.e., <domain>:<route>).  The path for a domain-name reservation is <domain>: 
	pub path: Path,

	/// The code hash for a smart contract, and optionally the underlying code of that contract
	pub contract: Option<Contract<T>>,
}

impl<T: frame_system::Config> RegisteredPath<T> {
	pub fn new(owner: &AccountIdOf<T>, path: &Path, contract: &Option<Contract<T>>) -> Self {
		Self {
			owner: owner.clone(),
			path: path.clone(),
			contract: contract.clone(),
		}
	}
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Maximum size for a domain name, in bytes
		#[pallet::constant]
		type MaxDomainByteSize: Get<u32>;

		/// Maximum size for a route name, in bytes
		#[pallet::constant]
		type MaxRouteByteSize: Get<u32>;
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	/// A table of registered paths (i.e., domain/route combinations).  A domain itself will always be registered with a route of None; 
	/// all other registered paths will have a Some<Vec<u8>> route string
	#[pallet::storage]
	pub type RegisteredPaths<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, Domain,
		Blake2_128Concat, Route,
		RegisteredPath<T>,
	>;

	/// A table of registered approvers for a given path, i.e., account(s) (if any) that have been approved by the path owner to also manage the path
	#[pallet::storage]
	pub type RegisteredApprovers<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, Domain,
		Blake2_128Concat, (Route, Approver<T>),
		(),
	>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(
		T::AccountId = "AccountId",
	)]
	pub enum Event<T: Config> {
		/// A name was set or reset (which will remove all judgements). \[who\]
		OwnerSet(T::AccountId, Domain),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Domain already exists.
		DomainAlreadyExists,
		/// Domain/Route Combination already exists.
		PathAlreadyExists,
		/// Domain name is too large.
		DomainNameTooLarge,
		/// Domain does not exist.
		DomainNotRegistered,
		/// Domain exists but route is not registered on the domain.
		RouteNotRegistered,
		/// The source of a call is not authorized to perform the call.
		CallerNotApproved,
		/// The path does not follow the <domain>:<slash-separated route> convention.
		MalformedPath,
		/// The domain does not follow the <domain>:<slash-separated route> convention.
		MalformedDomain,
	}

	impl<T: Config> Pallet<T> {	
		/// Return Some(registered path) if registered, None if not registered, or MalformedPath if the given domain and route are invalid
		pub fn get_registered_path_info(path: &Path) -> Option<RegisteredPath<T>> {
			if_std! { println!("in get_registered_path_info, looking for {:#?}", path.clone()); }
			RegisteredPaths::<T>::get(path.domain_lc.clone(), path.route_lc.clone())
		}

		/// Return true if registered, false if not (Malformed domains/routes are reported as false)
		pub fn is_registered_path(path: &Path) -> bool {
			match Pallet::<T>::get_registered_path_info(path) {
				Some(_) => true,
				_ => false
			}
		}

		/// Return true if registered, false if not (Malformed domains/routes are reported as false)
		pub fn is_registered_domain(path: &Path) -> bool {
			match Path::new(&path.domain_lc, &String::from("")) {
				Ok(domain_path) => Pallet::<T>::is_registered_path(&domain_path),
				_ => false
			}
		}

		/// Return true if registered, false if not (Malformed domains/routes are reported as false)
		pub fn is_registered_approver(path: &Path, approver: &Approver<T>) -> bool {
			match RegisteredApprovers::<T>::get(path.domain_lc.clone(), (path.route_lc.clone(), &approver)) {
				Some(_) => true,
				_ => false
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {	
		/// Register a domain.
		/// A domain must be registered before routes on that domain can be created.  register_domain will lower-case the string before
		#[pallet::weight(10_000)]
		pub fn register_domain(
			origin: OriginFor<T>,
			domain: String,
		) -> DispatchResultWithPostInfo {
			
			let owner = ensure_signed(origin)?;
			let null_route = String::from("");
			match Path::new(&domain, &null_route) {
				Ok(path) => match Pallet::<T>::is_registered_path(&path) {
					false => {
						let registered_path = RegisteredPath::new(&owner, &path, &None);
						if_std! { println!("in register_domain, Ok path, registered_path = {:#?}", registered_path); }
			
						// if_std! { println!("in register_domain, Adding the following K/K to RegisteredPaths = {:#?},  {:#?}", registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone()); }
						RegisteredPaths::<T>::insert(registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone(), registered_path.clone());
						// if_std! { println!("in register_domain, Adding the following K/K to RegisteredApprovers = {:#?},  {:#?}", registered_path.clone().path.domain_lc.clone(), (registered_path.clone().path.route_lc.clone(), owner.clone())); }
						RegisteredApprovers::<T>::insert(registered_path.clone().path.domain_lc.clone(), (registered_path.clone().path.route_lc.clone(), owner.clone()), ());
						Self::deposit_event(Event::OwnerSet(owner.clone(), registered_path.clone().path.domain_lc.clone()));
			
						Ok(Some(10_000).into())
					},
					true => Err(Error::<T>::DomainAlreadyExists)?,
				},
				Err(_) => Err(Error::<T>::MalformedDomain)?,
			}
		}

		/// Register a contract.
		/// A domain must be registered before contracts on that domain can be registered.
		#[pallet::weight(10_000)]
		pub fn register_contract(
			origin: OriginFor<T>,
			path_string: String,
			code_hash: CodeHashOf<T>,
			code: Option<Code>,
		) -> DispatchResultWithPostInfo {
			let caller: AccountIdOf<T> = ensure_signed(origin)?;
			match Path::new_from_path_string(&path_string) {				
				Ok(path) => match Pallet::<T>::is_registered_domain(&path) {
					true => match Pallet::<T>::is_registered_approver(&path, &caller) {
						true => {
							let registered_path = RegisteredPath::new(&caller, &path, &Some(Contract::new(&code_hash, &code)));
							if_std! { println!("in register_contract, Ok path, registered_path = {:#?}", registered_path); }
				
							if_std! { println!("in register_contract, Adding the following K/K to RegisteredPaths = {:#?},  {:#?}", registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone()); }
							RegisteredPaths::<T>::insert(registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone(), registered_path.clone());
							if_std! { println!("in register_contract, Adding the following K/K to RegisteredApprovers = {:#?},  {:#?}", registered_path.clone().path.route_lc.clone(), caller.clone()); }
							RegisteredApprovers::<T>::insert(registered_path.clone().path.domain_lc.clone(), (registered_path.clone().path.route_lc.clone(), caller.clone()), ());
							Self::deposit_event(Event::OwnerSet(caller.clone(), registered_path.clone().path.domain_lc.clone()));
				
							Ok(Some(10_000).into())		
						},
						false => Err(Error::<T>::CallerNotApproved)?,
					},
					false => Err(Error::<T>::DomainNotRegistered)?,
				},
				Err(_) => Err(Error::<T>::MalformedDomain)?,
			}
		}

		#[pallet::weight(10_000)]
		pub fn add_approver(
			origin: OriginFor<T>,
			path: String,
			approver: Approver<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			match Path::new_from_path_string(&path) {
				Ok(path) => match Pallet::<T>::is_registered_path(&path) {
					true => match Pallet::<T>::is_registered_approver(&path, &caller) {
						true => {
							RegisteredApprovers::<T>::insert(path.domain_lc.clone(), (path.route_lc.clone(), approver), ());
							Ok(Some(10_000).into())
						},
						false => Err(Error::<T>::CallerNotApproved)?,
					},
					false => Err(Error::<T>::DomainNotRegistered)?,
				},
				Err(_) => Err(Error::<T>::MalformedDomain)?,
			}
		}
	}
}

