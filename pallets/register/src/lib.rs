#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use sp_std::if_std;
use frame_support::{
	RuntimeDebugNoBound,
};
use codec::{Encode, Decode};

mod serstring;
use serstring::SerString;

#[cfg(test)]
mod tests;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

// pub enum PathGen<'a> {
//     /// A value domain/route combination, with domain D separate from route R.
//     DomainRoute(&'a SerString, &'a SerString),
//     /// A path, expressed as <domain> (if route is None) or <domain>:<route>.
//     Path(&'a SerString),
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
	pub domain: SerString,

	/// <domain> as a lower-cased string.
	pub domain_lc: SerString,

	/// <route> as a string with original case preserved.  
	/// A top-level domain has a route of "".
	/// Routes are also UTF-8 strings, where sub-routes within the route are slash-separated.
	/// Routes are case-insensitive; Order/Cookies and order/cookies are considered the same route.
	/// A route of None corresponds to a pure / top-level (i.e., a route-less) domain
	pub route: SerString,

	/// <route> as a lower-cased string.
	pub route_lc: SerString,

	/// <domain>: if route is none, else <domain>:<route>, as a string with original ca
	/// s
	/// 
	/// 
	/// e
	///  
	/// p
	/// 
	/// r
	/// eserved.
	pub path: SerString,

	/// <path> as a lower-cased string.
	pub path_lc: SerString,
}


impl Path {
	pub fn new(domain: SerString, route: SerString) -> Self {
		let domain = domain.replace(":", "");
		let domain_lc = domain.to_lowercase();
		// let route = route.clone();
		let route_lc = route.to_lowercase();
		let path = SerString::from_str(&[domain.to_str(), ":", route.to_str()].concat()[..]);
		let path_lc = path.to_lowercase();

		Self {
			domain,
			domain_lc,
			route,
			route_lc,
			path,
			path_lc,
		}
	}

	pub fn new_from_path(path: SerString) -> Result<Path, SerString> {
		let split_path = path.split(":");
		if (split_path.len() == 1) || (split_path.len() == 2) {
			let route: SerString = if split_path.len() == 1 {
				SerString::from_str("").into()
			} else {
				split_path[1].clone()
			};
			Ok(Self::new(split_path[0].clone(), route))
		} else {
			Err("Malformed Path".into())
		}
	}
}

// #[cfg(feature = "std")]
impl sp_std::fmt::Display for Path {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "Path {{ {} }}", self.path_lc.to_str())
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

// #[cfg(feature = "std")]
impl<T: frame_system::Config> sp_std::fmt::Display for Contract<T> {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "Contract {{ {:#?} }}", self.code_hash)
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

impl<T: frame_system::Config> sp_std::fmt::Display for RegisteredPath<T> {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		match &self.contract {
			Some(contract) => write!(f, "RegisteredPath {{owner: {:?}, path: {}, contract: {:?} }}", self.owner, self.path.path_lc, contract.code_hash),
			None => write!(f, "RegisteredPath {{owner: {:?}, path: {}, contract: None", self.owner, self.path.path_lc),
		}		
	}
}


pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
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
	/// all other registered paths will have a Some<SerString> route string
	#[pallet::storage]
	/// \[domain, route, path/owner registration info\]
	pub type RegisteredPaths<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, SerString,
		Blake2_128Concat, SerString,
		RegisteredPath<T>,
	>;

	/// A table of registered approvers for a given path, i.e., account(s) (if any) that have been approved by the path owner to also manage the path.
	/// \[domain, (route, approver)\]
	#[pallet::storage]
	pub type RegisteredApprovers<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, SerString,
		Blake2_128Concat, (SerString, Approver<T>),
		(),
	>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	#[pallet::metadata(
		T::AccountId = "AccountId",
	)]
	pub enum Event<T: Config> {
		/// An owner was set or reset. \[who, path\]
		OwnerSet(T::AccountId, SerString),
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

	// #[pallet::hooks]
	// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	// 	// TODO_MAYBE_WHERE_CLAUSE
	// {
	// 	// TODO_ON_FINALIZE
	// 	// TODO_ON_INITIALIZE
	// 	// TODO_ON_RUNTIME_UPGRADE
	// 	// TODO_INTEGRITY_TEST
	// 	// TODO_OFFCHAIN_WORKER
	// }

	impl<T: Config> Pallet<T> {	
		/// Return Some(registered path) if registered, None if not registered, or MalformedPath if the given domain and route are invalid
		pub fn get_registered_path_info(path: &Path) -> Option<RegisteredPath<T>> {
			if_std! { println!("in get_registered_path_info, looking for {}", path); }
			RegisteredPaths::<T>::get(path.domain_lc.clone(), path.route_lc.clone())
		}

		/// Return true if registered, false if not (Malformed domains/routes are reported as false)
		pub fn is_registered_path(path: &Path) -> bool {
			match Pallet::<T>::get_registered_path_info(path) {
				Some(_) => true,
				_ => false
			}
		}

		/// Return true if domain is registered, false if not
        pub fn is_registered_domain(path: &Path) -> bool {
            let route: SerString = "".into();
            match RegisteredPaths::<T>::get(path.domain_lc.clone(), route) {
                Some(_) => true,
                None => false
            }
        }
		/// Return true if approver is registered for path, false if not
		pub fn is_registered_approver(path: &Path, approver: &Approver<T>) -> bool {
			match RegisteredApprovers::<T>::get(path.domain_lc.clone(), (path.route_lc.clone(), &approver)) {
				Some(_) => true,
				_ => false
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	{
		/// Register a domain.
		/// A domain must be registered before routes on that domain can be created.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register_domain(
			origin: OriginFor<T>,
			domain: SerString,
		) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let null_route = SerString::from_str("");
			let path = Path::new(domain, null_route);
			match Pallet::<T>::is_registered_path(&path) {
				false => {
					let registered_path = RegisteredPath::new(&owner, &path, &None);
					if_std! { println!("in register_domain, Ok path, registered_path = {}", registered_path); }
		
					if_std! { println!("in register_domain, Adding the following K/K to RegisteredPaths = {},  {}", registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone()); }
					RegisteredPaths::<T>::insert(registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone(), registered_path.clone());
					if_std! { println!("in register_domain, Adding the following K/K to RegisteredApprovers = {}, ({}, {:?})", registered_path.clone().path.domain_lc.clone(), registered_path.clone().path.route_lc.clone(), owner.clone()); }
					RegisteredApprovers::<T>::insert(registered_path.clone().path.domain_lc.clone(), (registered_path.clone().path.route_lc.clone(), owner.clone()), ());
					Self::deposit_event(Event::OwnerSet(owner.clone(), registered_path.clone().path.domain_lc.clone()));
		
					Ok(Some(10_000).into())
				},
				true => Err(Error::<T>::DomainAlreadyExists)?,
			}
		}

		/// Register a contract.
		/// A domain must be registered before contracts on that domain can be registered.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn register_contract(
			origin: OriginFor<T>,
			path: SerString,
			code_hash: CodeHashOf<T>,
			code: Option<Code>,
		) -> DispatchResultWithPostInfo {
			let caller: AccountIdOf<T> = ensure_signed(origin)?;
			match Path::new_from_path(path.clone()) {				
				Ok(path) => match Pallet::<T>::is_registered_domain(&path) {
					true => {
						let domain_lc = path.clone().domain_lc;
						let routes = path.route_lc.split("/");
						let super_paths = (0..(routes.len() + 1)).map(|x| {
							match x {
								0 => Path::new_from_path(SerString::from_str(domain_lc.to_str())).unwrap(),
								_ => Path::new_from_path(SerString::join(vec![domain_lc.clone(), SerString::join(routes[0..x].to_vec(), "/")], ":")).unwrap(),
							}
						}).rev();
						let mut is_approved = false;
						let mut paths_to_potentially_be_approved = Vec::<Path>::new();
						for super_path in super_paths {
							if Pallet::<T>::is_registered_approver(&super_path, &caller) {
								// we have reached a path where the caller is approved, and we have a (potentially empty) list of additional paths
								// to be approved, so break with caller approved
								if_std! { println!("in register_contract, found approved path {} for caller {:?}", super_path, caller); }
								is_approved = true;
								break;
							} else {
								if Pallet::<T>::is_registered_path(&super_path) {
									// The path has already been registered but the caller is not approved on it, so they are disapproved to add a contract
									// at or below this point in the hierarchy, so leave is_approved == false and break
									if_std! { println!("in register_contract, found a registered path {} for whic caller {:?} is not approved", super_path, caller); }
									break;
								} else {
									// Since this path doesn't yet exist, if we find a super path where the caller is approved, then we'll want to
									// approve the caller for this sub path as well.  I.e., if caller A registers a domain "new_domain", they will
									// be the approver on "new_domain:".  If they then approve caller B on the path "new_domain:a", then if caller
									// B registers a contract at "new_domain:a/path/to/the/contract", then we want to approve caller B for that
									// path and also the paths "new_domain:a/path/to/the", "new_domain:a/path/to" and "new_domain:a/path".  We
									// don't want to approve caller B for "new_domain:a" (since they are already approved for that path) nor for "new_domain:"
									// (since caller A owns that path, only they have the right to approve caller B for it).__rust_force_expr!

									// Note that at this point, caller A does NOT have approvals on any of the sub-paths of "new_domain:a", only
									// caller B has them.  Caller B can approve caller A on those paths at their discretion, or call A can 
									if_std! { println!("in register_contract, found a path {} that potentially needs to be approved for caller {:?}", super_path, caller); }
									paths_to_potentially_be_approved.push(super_path.clone());
								}
							}
						}

						match is_approved {
							true => {
								if_std! { println!("in register_contract, caller {:?} is approved about to approve super paths", caller); }
								for ptba in paths_to_potentially_be_approved {
									if_std! { println!("in register_contract, path {} to be approved for caller {:?}", ptba, caller); }
									let contract = match ptba == path {
										true => Some(Contract::new(&code_hash, &code)),
										false => None
									};
									let registered_path = RegisteredPath::new(&caller, &ptba, &contract);
									if_std! { println!("in register_contract is_approved path, registered_path = {}", registered_path); }
									if_std! { println!("in register_contract is_approved path, Adding the following K/K to RegisteredPaths = {},  {}", registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone()); }
									RegisteredPaths::<T>::insert(registered_path.path.domain_lc.clone(), registered_path.clone().path.route_lc.clone(), registered_path.clone());
									if_std! { println!("in register_contract is_approved path, Adding the following K/K to RegisteredApprovers = {},  {:?}", registered_path.clone().path.route_lc.clone(), caller.clone()); }
									RegisteredApprovers::<T>::insert(registered_path.clone().path.domain_lc, (registered_path.clone().path.route_lc, caller.clone()), ());
									Self::deposit_event(Event::OwnerSet(caller.clone(), registered_path.clone().path.domain_lc));
								}
								Ok(Some(10_000).into())		
							},
							false => {
								if_std! { println!("in register_contract, caller not approved, path = {}, caller = {}", path, caller); }; 
								Err(Error::<T>::CallerNotApproved)?
							},
						}
					},
					false => {
						if_std! { println!("in register_contract, domain not registered, path = {}", path); }; 
						Err(Error::<T>::DomainNotRegistered)?
					},
				},
				Err(_) => {
					if_std! { println!("in register_contract, malformed domain, path = {}", path.clone()); }; 
					Err(Error::<T>::MalformedDomain)?
				},
			}
		}

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_approver(
			origin: OriginFor<T>,
			path: SerString,
			approver: Approver<T>,
		) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;
			match Path::new_from_path(path) {
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
