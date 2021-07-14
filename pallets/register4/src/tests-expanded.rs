#![feature(prelude_import)]
#![cfg(test)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;


// to produce this file, execute:  cargo expand --color never --release --lib --tests > src/tests-expanded.rs


mod tests {
    use super::*;
    use crate as pallet_compose_register;
    use frame_support::{assert_ok, assert_noop, parameter_types};
    use sp_core::H256;
    use sp_runtime::{
        testing::Header,
        traits::{BlakeTwo256, IdentityLookup},
    };
    type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
    type Block = frame_system::mocking::MockBlock<Test>;
    #[doc(hidden)]
    mod sp_api_hidden_includes_construct_runtime {
        pub extern crate frame_support as hidden_include;
    }
    const _: () = {
        #[allow(unused)]
        type __hidden_use_of_unchecked_extrinsic = UncheckedExtrinsic;
    };
    pub struct Test;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Test {
        #[inline]
        fn clone(&self) -> Test {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Test {}
    impl ::core::marker::StructuralPartialEq for Test {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Test {
        #[inline]
        fn eq(&self, other: &Test) -> bool {
            match *other {
                Test => match *self {
                    Test => true,
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for Test {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Test {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl core::fmt::Debug for Test {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("Test").finish()
        }
    }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetNodeBlockType for Test { type NodeBlock = Block ; }
    impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetRuntimeBlockType for Test { type RuntimeBlock = Block ; }
    #[allow(non_camel_case_types)]
    pub enum Event {
        #[codec(index = 0u8)]
        System(frame_system::Event<Test>),
        #[codec(index = 1u8)]
        Balances(pallet_balances::Event<Test>),
        #[codec(index = 2u8)]
        ComposeRegister(pallet_compose_register::Event<Test>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for Event {
        #[inline]
        fn clone(&self) -> Event {
            match (&*self,) {
                (&Event::System(ref __self_0),) => {
                    Event::System(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Event::Balances(ref __self_0),) => {
                    Event::Balances(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Event::ComposeRegister(ref __self_0),) => {
                    Event::ComposeRegister(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for Event {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for Event {
        #[inline]
        fn eq(&self, other: &Event) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Event::Balances(ref __self_0), &Event::Balances(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (
                            &Event::ComposeRegister(ref __self_0),
                            &Event::ComposeRegister(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Event) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Event::Balances(ref __self_0), &Event::Balances(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (
                            &Event::ComposeRegister(ref __self_0),
                            &Event::ComposeRegister(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralEq for Event {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for Event {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Test>>;
                let _: ::core::cmp::AssertParamIsEq<pallet_balances::Event<Test>>;
                let _: ::core::cmp::AssertParamIsEq<pallet_compose_register::Event<Test>>;
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Event {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::System(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Event::Balances(ref aa) => {
                        __codec_dest_edqy.push_byte(1u8 as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Event::ComposeRegister(ref aa) => {
                        __codec_dest_edqy.push_byte(2u8 as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl _parity_scale_codec::EncodeLike for Event {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Event {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as u8 => Ok(Event::System({
                        let __codec_res_edqy =
                            <frame_system::Event<Test> as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Event::System.0`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    __codec_x_edqy if __codec_x_edqy == 1u8 as u8 => Ok(Event::Balances({
                        let __codec_res_edqy =
                            <pallet_balances::Event<Test> as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Event::Balances.0`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    __codec_x_edqy if __codec_x_edqy == 2u8 as u8 => Ok(Event::ComposeRegister({
                        let __codec_res_edqy = < pallet_compose_register :: Event < Test > as _parity_scale_codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `Event::ComposeRegister.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    _ => Err("Could not decode `Event`, variant doesn\'t exist".into()),
                }
            }
        }
    };
    impl core::fmt::Debug for Event {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::System(ref a0) => fmt.debug_tuple("Event::System").field(a0).finish(),
                Self::Balances(ref a0) => fmt.debug_tuple("Event::Balances").field(a0).finish(),
                Self::ComposeRegister(ref a0) => {
                    fmt.debug_tuple("Event::ComposeRegister").field(a0).finish()
                }
                _ => Ok(()),
            }
        }
    }
    impl From<frame_system::Event<Test>> for Event {
        fn from(x: frame_system::Event<Test>) -> Self {
            Event::System(x)
        }
    }
    impl
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
            frame_system::Event<Test>,
        > for Event
    {
        type Error = ();        fn try_into (self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < frame_system :: Event < Test > , Self :: Error >{
            match self {
                Self::System(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }
    impl From<pallet_balances::Event<Test>> for Event {
        fn from(x: pallet_balances::Event<Test>) -> Self {
            Event::Balances(x)
        }
    }
    impl
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
            pallet_balances::Event<Test>,
        > for Event
    {
        type Error = ();        fn try_into (self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < pallet_balances :: Event < Test > , Self :: Error >{
            match self {
                Self::Balances(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }
    impl From<pallet_compose_register::Event<Test>> for Event {
        fn from(x: pallet_compose_register::Event<Test>) -> Self {
            Event::ComposeRegister(x)
        }
    }
    impl
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryInto<
            pallet_compose_register::Event<Test>,
        > for Event
    {
        type Error = ();        fn try_into (self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < pallet_compose_register :: Event < Test > , Self :: Error >{
            match self {
                Self::ComposeRegister(evt) => Ok(evt),
                _ => Err(()),
            }
        }
    }
    pub struct Origin {
        caller: OriginCaller,
        filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<
            Box<dyn Fn(&<Test as frame_system::Config>::Call) -> bool>,
        >,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Origin {
        #[inline]
        fn clone(&self) -> Origin {
            match *self {
                Origin {
                    caller: ref __self_0_0,
                    filter: ref __self_0_1,
                } => Origin {
                    caller: ::core::clone::Clone::clone(&(*__self_0_0)),
                    filter: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    #[cfg(feature = "std")]
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug for Origin {
        fn fmt (& self , fmt : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: fmt :: Formatter) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < () , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: fmt :: Error >{
            fmt.debug_struct("Origin")
                .field("caller", &self.caller)
                .field("filter", &"[function ptr]")
                .finish()
        }
    }
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait
        for Origin
    {
        type Call = <Test as frame_system::Config>::Call;
        type PalletsOrigin = OriginCaller;
        type AccountId = <Test as frame_system::Config>::AccountId;
        fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
            let f = self.filter.clone();
            self.filter =
                self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(move |call| f(call) && filter(call)),
                );
        }
        fn reset_filter(&mut self) {
            let filter = < < Test as frame_system :: Config > :: BaseCallFilter as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: Filter < < Test as frame_system :: Config > :: Call > > :: filter ;
            self.filter =
                self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(filter),
                );
        }
        fn set_caller_from(&mut self, other: impl Into<Self>) {
            self.caller = other.into().caller;
        }
        fn filter_call(&self, call: &Self::Call) -> bool {
            (self.filter)(call)
        }
        fn caller(&self) -> &Self::PalletsOrigin {
            &self.caller
        }
        fn try_with_caller<R>(
            mut self,
            f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin>,
        ) -> Result<R, Self> {
            match f(self.caller) {
                Ok(r) => Ok(r),
                Err(caller) => {
                    self.caller = caller;
                    Err(self)
                }
            }
        }
        /// Create with system none origin and `frame-system::Config::BaseCallFilter`.
        fn none() -> Self {
            frame_system::RawOrigin::None.into()
        }
        /// Create with system root origin and no filter.
        fn root() -> Self {
            frame_system::RawOrigin::Root.into()
        }
        /// Create with system signed origin and `frame-system::Config::BaseCallFilter`.
        fn signed(by: <Test as frame_system::Config>::AccountId) -> Self {
            frame_system::RawOrigin::Signed(by).into()
        }
    }
    #[allow(non_camel_case_types)]
    pub enum OriginCaller {
        #[codec(index = 0u8)]
        system(frame_system::Origin<Test>),
        #[allow(dead_code)]
        Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::clone::Clone for OriginCaller {
        #[inline]
        fn clone(&self) -> OriginCaller {
            match (&*self,) {
                (&OriginCaller::system(ref __self_0),) => {
                    OriginCaller::system(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&OriginCaller::Void(ref __self_0),) => {
                    OriginCaller::Void(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for OriginCaller {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for OriginCaller {
        #[inline]
        fn eq(&self, other: &OriginCaller) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &OriginCaller::system(ref __self_0),
                            &OriginCaller::system(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &OriginCaller) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &OriginCaller::system(ref __self_0),
                            &OriginCaller::system(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralEq for OriginCaller {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::Eq for OriginCaller {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Test>>;
                let _: ::core::cmp::AssertParamIsEq<
                    self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
                >;
            }
        }
    }
    impl core::fmt::Debug for OriginCaller {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::system(ref a0) => fmt.debug_tuple("OriginCaller::system").field(a0).finish(),
                Self::Void(ref a0) => fmt.debug_tuple("OriginCaller::Void").field(a0).finish(),
                _ => Ok(()),
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for OriginCaller {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    OriginCaller::system(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    OriginCaller::Void(ref aa) => {
                        __codec_dest_edqy.push_byte(1usize as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl _parity_scale_codec::EncodeLike for OriginCaller {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for OriginCaller {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy.read_byte().map_err(|e| {
                    e.chain("Could not decode `OriginCaller`, failed to read variant byte")
                })? {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as u8 => Ok(OriginCaller::system({
                        let __codec_res_edqy =
                            <frame_system::Origin<Test> as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `OriginCaller::system.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    __codec_x_edqy if __codec_x_edqy == 1usize as u8 => Ok(OriginCaller::Void({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void as _parity_scale_codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `OriginCaller::Void.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    _ => Err("Could not decode `OriginCaller`, variant doesn\'t exist".into()),
                }
            }
        }
    };
    #[allow(dead_code)]
    impl Origin {
        /// Create with system none origin and `frame-system::Config::BaseCallFilter`.
        pub fn none() -> Self {
            < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: none ()
        }
        /// Create with system root origin and no filter.
        pub fn root() -> Self {
            < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: root ()
        }
        /// Create with system signed origin and `frame-system::Config::BaseCallFilter`.
        pub fn signed(by: <Test as frame_system::Config>::AccountId) -> Self {
            < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: signed (by)
        }
    }
    impl From<frame_system::Origin<Test>> for OriginCaller {
        fn from(x: frame_system::Origin<Test>) -> Self {
            OriginCaller::system(x)
        }
    }
    impl
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::convert::TryFrom<
            OriginCaller,
        > for frame_system::Origin<Test>
    {
        type Error = OriginCaller;        fn try_from (x : OriginCaller) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result < frame_system :: Origin < Test > , OriginCaller >{
            if let OriginCaller::system(l) = x {
                Ok(l)
            } else {
                Err(x)
            }
        }
    }
    impl From<frame_system::Origin<Test>> for Origin {
        /// Convert to runtime origin:
        /// * root origin is built with no filter
        /// * others use `frame-system::Config::BaseCallFilter`
        fn from(x: frame_system::Origin<Test>) -> Self {
            let o: OriginCaller = x.into();
            o.into()
        }
    }
    impl From<OriginCaller> for Origin {
        fn from(x: OriginCaller) -> Self {
            let mut o = Origin { caller : x , filter : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: rc :: Rc :: new (Box :: new (| _ | true)) , } ;
            if !match o.caller {
                OriginCaller::system(frame_system::Origin::<Test>::Root) => true,
                _ => false,
            } {
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait :: reset_filter (& mut o) ;
            }
            o
        }
    }
    impl From<Origin>
        for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
            frame_system::Origin<Test>,
            Origin,
        >
    {
        /// NOTE: converting to pallet origin loses the origin filter information.
        fn from(val: Origin) -> Self {
            if let OriginCaller::system(l) = val.caller {
                Ok(l)
            } else {
                Err(val)
            }
        }
    }
    impl From<Option<<Test as frame_system::Config>::AccountId>> for Origin {
        /// Convert to runtime origin with caller being system signed or none and use filter
        /// `frame-system::Config::BaseCallFilter`.
        fn from(x: Option<<Test as frame_system::Config>::AccountId>) -> Self {
            <frame_system::Origin<Test>>::from(x).into()
        }
    }
    pub type System = frame_system::Pallet<Test>;
    pub type Balances = pallet_balances::Pallet<Test>;
    pub type ComposeRegister = pallet_compose_register::Pallet<Test>;
    /// All pallets included in the runtime as a nested tuple of types.
    /// Excludes the System pallet.
    pub type AllPallets = ((ComposeRegister, (Balances,)));
    /// All pallets included in the runtime as a nested tuple of types.
    pub type AllPalletsWithSystem = ((ComposeRegister, (Balances, (System,))));
    /// All modules included in the runtime as a nested tuple of types.
    /// Excludes the System pallet.
    #[deprecated(note = "use `AllPallets` instead")]
    #[allow(dead_code)]
    pub type AllModules = ((ComposeRegister, (Balances,)));
    /// All modules included in the runtime as a nested tuple of types.
    #[deprecated(note = "use `AllPalletsWithSystem` instead")]
    #[allow(dead_code)]
    pub type AllModulesWithSystem = ((ComposeRegister, (Balances, (System,))));
    /// Provides an implementation of `PalletInfo` to provide information
    /// about the pallet setup in the runtime.
    pub struct PalletInfo;
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
        for PalletInfo
    {
        fn index<P: 'static>() -> Option<usize> {
            let type_id = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < P > () ;
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some (0usize) }
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some (1usize) }
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < ComposeRegister > () { return Some (2usize) }
            None
        }
        fn name<P: 'static>() -> Option<&'static str> {
            let type_id = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < P > () ;
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some ("System") }
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < Balances > () { return Some ("Balances") }
            if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < ComposeRegister > () { return Some ("ComposeRegister") }
            None
        }
    }
    pub enum Call {
        #[codec(index = 0u8)]
        System(::frame_support::dispatch::CallableCallFor<System, Test>),
        #[codec(index = 1u8)]
        Balances(::frame_support::dispatch::CallableCallFor<Balances, Test>),
        #[codec(index = 2u8)]
        ComposeRegister(::frame_support::dispatch::CallableCallFor<ComposeRegister, Test>),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Call {
        #[inline]
        fn clone(&self) -> Call {
            match (&*self,) {
                (&Call::System(ref __self_0),) => {
                    Call::System(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Call::Balances(ref __self_0),) => {
                    Call::Balances(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Call::ComposeRegister(ref __self_0),) => {
                    Call::ComposeRegister(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Call {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Call {
        #[inline]
        fn eq(&self, other: &Call) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                            (*__self_0) == (*__arg_1_0)
                        }
                        (
                            &Call::ComposeRegister(ref __self_0),
                            &Call::ComposeRegister(ref __arg_1_0),
                        ) => (*__self_0) == (*__arg_1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &Call) -> bool {
            {
                let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                            (*__self_0) != (*__arg_1_0)
                        }
                        (
                            &Call::ComposeRegister(ref __self_0),
                            &Call::ComposeRegister(ref __arg_1_0),
                        ) => (*__self_0) != (*__arg_1_0),
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    impl ::core::marker::StructuralEq for Call {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Call {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<
                    ::frame_support::dispatch::CallableCallFor<System, Test>,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    ::frame_support::dispatch::CallableCallFor<Balances, Test>,
                >;
                let _: ::core::cmp::AssertParamIsEq<
                    ::frame_support::dispatch::CallableCallFor<ComposeRegister, Test>,
                >;
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for Call {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::System(ref aa) => {
                        __codec_dest_edqy.push_byte(0u8 as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::Balances(ref aa) => {
                        __codec_dest_edqy.push_byte(1u8 as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    Call::ComposeRegister(ref aa) => {
                        __codec_dest_edqy.push_byte(2u8 as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl _parity_scale_codec::EncodeLike for Call {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for Call {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as u8 => Ok(Call::System({
                        let __codec_res_edqy = <::frame_support::dispatch::CallableCallFor<
                            System,
                            Test,
                        > as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Call::System.0`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    __codec_x_edqy if __codec_x_edqy == 1u8 as u8 => Ok(Call::Balances({
                        let __codec_res_edqy = <::frame_support::dispatch::CallableCallFor<
                            Balances,
                            Test,
                        > as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Call::Balances.0`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    __codec_x_edqy if __codec_x_edqy == 2u8 as u8 => Ok(Call::ComposeRegister({
                        let __codec_res_edqy = <::frame_support::dispatch::CallableCallFor<
                            ComposeRegister,
                            Test,
                        > as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy
                        );
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `Call::ComposeRegister.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    })),
                    _ => Err("Could not decode `Call`, variant doesn\'t exist".into()),
                }
            }
        }
    };
    impl core::fmt::Debug for Call {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self {
                Self::System(ref a0) => fmt.debug_tuple("Call::System").field(a0).finish(),
                Self::Balances(ref a0) => fmt.debug_tuple("Call::Balances").field(a0).finish(),
                Self::ComposeRegister(ref a0) => {
                    fmt.debug_tuple("Call::ComposeRegister").field(a0).finish()
                }
                _ => Ok(()),
            }
        }
    }
    impl ::frame_support::dispatch::GetDispatchInfo for Call {
        fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
            match self {
                Call::System(call) => call.get_dispatch_info(),
                Call::Balances(call) => call.get_dispatch_info(),
                Call::ComposeRegister(call) => call.get_dispatch_info(),
            }
        }
    }
    impl ::frame_support::dispatch::GetCallMetadata for Call {
        fn get_call_metadata(&self) -> ::frame_support::dispatch::CallMetadata {
            use ::frame_support::dispatch::GetCallName;
            match self {
                Call::System(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "System";
                    ::frame_support::dispatch::CallMetadata {
                        function_name,
                        pallet_name,
                    }
                }
                Call::Balances(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "Balances";
                    ::frame_support::dispatch::CallMetadata {
                        function_name,
                        pallet_name,
                    }
                }
                Call::ComposeRegister(call) => {
                    let function_name = call.get_call_name();
                    let pallet_name = "ComposeRegister";
                    ::frame_support::dispatch::CallMetadata {
                        function_name,
                        pallet_name,
                    }
                }
            }
        }
        fn get_module_names() -> &'static [&'static str] {
            &["System", "Balances", "ComposeRegister"]
        }
        fn get_call_names(module: &str) -> &'static [&'static str] {
            use ::frame_support::dispatch::{Callable, GetCallName};
            match module {
                "System" => <<System as Callable<Test>>::Call as GetCallName>::get_call_names(),
                "Balances" => <<Balances as Callable<Test>>::Call as GetCallName>::get_call_names(),
                "ComposeRegister" => {
                    <<ComposeRegister as Callable<Test>>::Call as GetCallName>::get_call_names()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    impl ::frame_support::dispatch::Dispatchable for Call {
        type Origin = Origin;
        type Config = Call;
        type Info = ::frame_support::weights::DispatchInfo;
        type PostInfo = ::frame_support::weights::PostDispatchInfo;
        fn dispatch(self, origin: Origin) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
            if !<Self::Origin as ::frame_support::traits::OriginTrait>::filter_call(&origin, &self)
            {
                return ::frame_support::sp_std::result::Result::Err(
                    ::frame_support::dispatch::DispatchError::BadOrigin.into(),
                );
            }
            ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(self, origin)
        }
    }
    impl ::frame_support::traits::UnfilteredDispatchable for Call {
        type Origin = Origin;
        fn dispatch_bypass_filter(
            self,
            origin: Origin,
        ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Call::System(call) => {
                    ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        call, origin,
                    )
                }
                Call::Balances(call) => {
                    ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        call, origin,
                    )
                }
                Call::ComposeRegister(call) => {
                    ::frame_support::traits::UnfilteredDispatchable::dispatch_bypass_filter(
                        call, origin,
                    )
                }
            }
        }
    }
    impl
        ::frame_support::traits::IsSubType<::frame_support::dispatch::CallableCallFor<System, Test>>
        for Call
    {
        #[allow(unreachable_patterns)]
        fn is_sub_type(&self) -> Option<&::frame_support::dispatch::CallableCallFor<System, Test>> {
            match *self {
                Call::System(ref r) => Some(r),
                _ => None,
            }
        }
    }
    impl From<::frame_support::dispatch::CallableCallFor<System, Test>> for Call {
        fn from(call: ::frame_support::dispatch::CallableCallFor<System, Test>) -> Self {
            Call::System(call)
        }
    }
    impl
        ::frame_support::traits::IsSubType<
            ::frame_support::dispatch::CallableCallFor<Balances, Test>,
        > for Call
    {
        #[allow(unreachable_patterns)]
        fn is_sub_type(
            &self,
        ) -> Option<&::frame_support::dispatch::CallableCallFor<Balances, Test>> {
            match *self {
                Call::Balances(ref r) => Some(r),
                _ => None,
            }
        }
    }
    impl From<::frame_support::dispatch::CallableCallFor<Balances, Test>> for Call {
        fn from(call: ::frame_support::dispatch::CallableCallFor<Balances, Test>) -> Self {
            Call::Balances(call)
        }
    }
    impl
        ::frame_support::traits::IsSubType<
            ::frame_support::dispatch::CallableCallFor<ComposeRegister, Test>,
        > for Call
    {
        #[allow(unreachable_patterns)]
        fn is_sub_type(
            &self,
        ) -> Option<&::frame_support::dispatch::CallableCallFor<ComposeRegister, Test>> {
            match *self {
                Call::ComposeRegister(ref r) => Some(r),
                _ => None,
            }
        }
    }
    impl From<::frame_support::dispatch::CallableCallFor<ComposeRegister, Test>> for Call {
        fn from(call: ::frame_support::dispatch::CallableCallFor<ComposeRegister, Test>) -> Self {
            Call::ComposeRegister(call)
        }
    }
    impl Test {
        pub fn metadata () -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataPrefixed{
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataLastVersion { modules : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("System") , index : 0u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Test > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Test > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Event :: < Test > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (frame_system :: Pallet :: < Test > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< frame_system :: Pallet < Test > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Balances") , index : 1u8 , storage : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Pallet :: < Test > :: storage_metadata))) , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Pallet :: < Test > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Event :: < Test > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_balances :: Pallet :: < Test > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_balances :: Pallet < Test > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleMetadata { name : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode ("ComposeRegister") , index : 2u8 , storage : None , calls : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_compose_register :: Pallet :: < Test > :: call_functions))) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_compose_register :: Event :: < Test > :: metadata))) , constants : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (pallet_compose_register :: Pallet :: < Test > :: module_constants_metadata)) , errors : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: FnEncode (< pallet_compose_register :: Pallet < Test > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ModuleErrorMetadata > :: metadata)) , }]) , extrinsic : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ExtrinsicMetadata { version : < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: VERSION , signed_extensions : < < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: SignedExtensions as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: SignedExtension > :: identifier () . into_iter () . map (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: DecodeDifferent :: Encode) . collect () , } , } . into ()
        }
    }
    #[cfg(any(feature = "std", test))]
    pub type SystemConfig = frame_system::GenesisConfig;
    #[cfg(any(feature = "std", test))]
    pub type BalancesConfig = pallet_balances::GenesisConfig<Test>;
    #[cfg(any(feature = "std", test))]
    use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde as __genesis_config_serde_import__;
    #[cfg(any(feature = "std", test))]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(crate = "__genesis_config_serde_import__")]
    pub struct GenesisConfig {
        pub system: SystemConfig,
        pub balances: BalancesConfig,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __genesis_config_serde_import__ as _serde;
        #[automatically_derived]
        impl __genesis_config_serde_import__::Serialize for GenesisConfig {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> __genesis_config_serde_import__::__private::Result<__S::Ok, __S::Error>
            where
                __S: __genesis_config_serde_import__::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GenesisConfig",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "system",
                    &self.system,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "balances",
                    &self.balances,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use __genesis_config_serde_import__ as _serde;
        #[automatically_derived]
        impl<'de> __genesis_config_serde_import__::Deserialize<'de> for GenesisConfig {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> __genesis_config_serde_import__::__private::Result<Self, __D::Error>
            where
                __D: __genesis_config_serde_import__::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"field index 0 <= i < 2",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "system" => _serde::__private::Ok(__Field::__field0),
                            "balances" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"system" => _serde::__private::Ok(__Field::__field0),
                            b"balances" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_field(
                                    __value, FIELDS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<GenesisConfig>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = GenesisConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct GenesisConfig")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<SystemConfig>(
                            &mut __seq,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GenesisConfig with 2 elements",
                                ));
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            BalancesConfig,
                        >(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct GenesisConfig with 2 elements",
                                ));
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            system: __field0,
                            balances: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<SystemConfig> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<BalancesConfig> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "system",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<SystemConfig>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "balances",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<BalancesConfig>(
                                            &mut __map,
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("system") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("balances") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            system: __field0,
                            balances: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["system", "balances"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GenesisConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GenesisConfig>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for GenesisConfig {
        #[inline]
        fn default() -> GenesisConfig {
            GenesisConfig {
                system: ::core::default::Default::default(),
                balances: ::core::default::Default::default(),
            }
        }
    }
    #[cfg(any(feature = "std", test))]
    impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage
        for GenesisConfig
    {
        fn assimilate_storage(
            &self,
            storage : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: Storage,
        ) -> std::result::Result<(), String> {
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Test , frame_system :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . system , storage) ? ;
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Test , pallet_balances :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . balances , storage) ? ;
            self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: BasicExternalities :: execute_with_storage (storage , | | { < AllPalletsWithSystem as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OnGenesis > :: on_genesis () ; }) ;
            Ok(())
        }
    }
    trait InherentDataExt {
        fn create_extrinsics(
            &self,
        ) -> ::frame_support::inherent::Vec<<Block as ::frame_support::inherent::BlockT>::Extrinsic>;
        fn check_extrinsics(
            &self,
            block: &Block,
        ) -> ::frame_support::inherent::CheckInherentsResult;
    }
    impl InherentDataExt for ::frame_support::inherent::InherentData {
        fn create_extrinsics(
            &self,
        ) -> ::frame_support::inherent::Vec<<Block as ::frame_support::inherent::BlockT>::Extrinsic>
        {
            use ::frame_support::inherent::ProvideInherent;
            let mut inherents = Vec::new();
            inherents
        }
        fn check_extrinsics(
            &self,
            block: &Block,
        ) -> ::frame_support::inherent::CheckInherentsResult {
            use ::frame_support::inherent::{ProvideInherent, IsFatalError};
            use ::frame_support::traits::{IsSubType, ExtrinsicCall};
            use ::frame_support::sp_runtime::traits::Block as _;
            let mut result = ::frame_support::inherent::CheckInherentsResult::new();
            for xt in block.extrinsics() {
                if ::frame_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false) {
                    break;
                }
                let mut is_inherent = false;
                if !is_inherent {
                    break;
                }
            }
            result
        }
    }
    impl ::frame_support::traits::EnsureInherentsAreFirst<Block> for Test {
        fn ensure_inherents_are_first(block: &Block) -> Result<(), u32> {
            use ::frame_support::inherent::ProvideInherent;
            use ::frame_support::traits::{IsSubType, ExtrinsicCall};
            use ::frame_support::sp_runtime::traits::Block as _;
            let mut first_signed_observed = false;
            for (i, xt) in block.extrinsics().iter().enumerate() {
                let is_signed =
                    ::frame_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false);
                let is_inherent = if is_signed {
                    false
                } else {
                    let mut is_inherent = false;
                    is_inherent
                };
                if !is_inherent {
                    first_signed_observed = true;
                }
                if first_signed_observed && is_inherent {
                    return Err(i as u32);
                }
            }
            Ok(())
        }
    }
    impl ::frame_support::unsigned::ValidateUnsigned for Test {
        type Call = Call;
        fn pre_dispatch(
            call: &Self::Call,
        ) -> Result<(), ::frame_support::unsigned::TransactionValidityError> {
            #[allow(unreachable_patterns)]
            match call {
                _ => Ok(()),
            }
        }
        fn validate_unsigned(
            #[allow(unused_variables)] source: ::frame_support::unsigned::TransactionSource,
            call: &Self::Call,
        ) -> ::frame_support::unsigned::TransactionValidity {
            #[allow(unreachable_patterns)]
            match call {
                _ => ::frame_support::unsigned::UnknownTransaction::NoUnsignedValidator.into(),
            }
        }
    }
    #[cfg(test)]
    mod __construct_runtime_integrity_test {
        use super::*;
        extern crate test;
        #[cfg(test)]
        #[rustc_test_marker]
        pub const runtime_integrity_tests: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "tests::__construct_runtime_integrity_test::runtime_integrity_tests",
                ),
                ignore: false,
                allow_fail: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(|| test::assert_test_result(runtime_integrity_tests())),
        };
        pub fn runtime_integrity_tests() {
            < AllPallets as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: IntegrityTest > :: integrity_test () ;
        }
    }
    pub struct BlockHashCount;
    impl BlockHashCount {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub const fn get() -> u64 {
            250
        }
    }
    impl<I: From<u64>> ::frame_support::traits::Get<I> for BlockHashCount {
        fn get() -> I {
            I::from(250)
        }
    }
    pub struct BlockWeights;
    impl BlockWeights {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub fn get() -> frame_system::limits::BlockWeights {
            frame_system::limits::BlockWeights::simple_max(1024)
        }
    }
    impl<I: From<frame_system::limits::BlockWeights>> ::frame_support::traits::Get<I> for BlockWeights {
        fn get() -> I {
            I::from(frame_system::limits::BlockWeights::simple_max(1024))
        }
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
    pub struct ExistentialDeposit;
    impl ExistentialDeposit {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub const fn get() -> u64 {
            1
        }
    }
    impl<I: From<u64>> ::frame_support::traits::Get<I> for ExistentialDeposit {
        fn get() -> I {
            I::from(1)
        }
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
    pub struct MaxDomainByteSize;
    impl MaxDomainByteSize {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub const fn get() -> u32 {
            32
        }
    }
    impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxDomainByteSize {
        fn get() -> I {
            I::from(32)
        }
    }
    pub struct MaxRouteByteSize;
    impl MaxRouteByteSize {
        /// Returns the value of this parameter type.
        #[allow(unused)]
        pub const fn get() -> u32 {
            512
        }
    }
    impl<I: From<u32>> ::frame_support::traits::Get<I> for MaxRouteByteSize {
        fn get() -> I {
            I::from(512)
        }
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
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();
        let genesis = pallet_balances::GenesisConfig::<Test> {
            balances: <[_]>::into_vec(box [(ACCOUNT_A, 100), (ACCOUNT_B, 200), (ACCOUNT_C, 300)]),
        };
        genesis.assimilate_storage(&mut t).unwrap();
        t.into()
    }
    const DOMAIN_X: &str = "MyDomainX";
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const successful_register: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::successful_register"),
            ignore: false,
            allow_fail: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(successful_register())),
    };
    fn successful_register() {
        let mut chain = new_test_ext();
        chain.execute_with(|| {
            let domain = String::from(DOMAIN_X);
            let route = String::from("");
            let path = new_path(PathGen::DomainRoute(domain.clone(), route)).unwrap();
            let goal_result: RegisteredPath<Test> = RegisteredPath {
                owner: ACCOUNT_A,
                path,
                code_hash: None,
                code: None,
            };
            let is = ComposeRegister::register_domain(
                Origin::signed(goal_result.owner),
                domain.clone(),
                goal_result.code_hash,
                goal_result.code.clone(),
            );
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &match (&is,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Debug::fmt,
                                    )],
                                },
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                            ))
                        }
                    }
                }
            };
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                    &[
                        "in register_domain, Looking for the following K/K in RegisteredPaths = ",
                        "\n   ",
                        "\n",
                    ],
                    &match (
                        &goal_result.path.domain.clone(),
                        &goal_result.path.route.clone(),
                    ) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                        ::core::fmt::rt::v1::Argument {
                            position: 1usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                    ],
                ));
            };
            {
                match (
                    &ComposeRegister::get_registered_path_info(
                        goal_result.path.domain.clone(),
                        goal_result.path.route.clone(),
                    )
                    .unwrap(),
                    &Some(goal_result.clone()),
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
            let h = ::frame_support::storage_root();
            {
                match (
                    &ComposeRegister::register_domain(
                        Origin::signed(goal_result.owner),
                        domain.clone(),
                        goal_result.code_hash,
                        goal_result.code.clone(),
                    ),
                    &Err(Error::<Test>::DomainAlreadyExists.into()),
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
            {
                match (&h, &::frame_support::storage_root()) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
            let h = ::frame_support::storage_root();
            {
                match (
                    &ComposeRegister::register_domain(
                        Origin::signed(ACCOUNT_B),
                        domain.clone(),
                        goal_result.code_hash,
                        goal_result.code.clone(),
                    ),
                    &Err(Error::<Test>::DomainAlreadyExists.into()),
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
            {
                match (&h, &::frame_support::storage_root()) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
        });
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const successful_approval: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::successful_approval"),
            ignore: false,
            allow_fail: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(successful_approval())),
    };
    fn successful_approval() {
        let mut chain = new_test_ext();
        chain.execute_with(|| {
            let domain = String::from(DOMAIN_X);
            let route = String::from("");
            let path = new_path(PathGen::DomainRoute(domain.clone(), route)).unwrap();
            let goal_result: RegisteredPath<Test> = RegisteredPath {
                owner: ACCOUNT_A,
                path,
                code_hash: None,
                code: None,
            };
            let is = ComposeRegister::register_domain(
                Origin::signed(goal_result.owner),
                domain.clone(),
                goal_result.code_hash,
                goal_result.code,
            );
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &match (&is,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Debug::fmt,
                                    )],
                                },
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                            ))
                        }
                    }
                }
            };
            let is = ComposeRegister::add_approver(
                Origin::signed(goal_result.owner),
                domain.clone(),
                ACCOUNT_B,
            );
            match is {
                Ok(_) => (),
                _ => {
                    if !false {
                        {
                            ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1_formatted(
                                &["Expected Ok(_). Got "],
                                &match (&is,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Debug::fmt,
                                    )],
                                },
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                            ))
                        }
                    }
                }
            };
            {
                match (
                    &ComposeRegister::is_registered_approver(
                        goal_result.path.domain.clone(),
                        goal_result.path.route.clone(),
                        ACCOUNT_A,
                    ),
                    &true,
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
            {
                match (
                    &ComposeRegister::is_registered_approver(
                        goal_result.path.domain.clone(),
                        goal_result.path.route.clone(),
                        ACCOUNT_B,
                    ),
                    &true,
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
            {
                match (
                    &ComposeRegister::is_registered_approver(
                        goal_result.path.domain.clone(),
                        goal_result.path.route.clone(),
                        ACCOUNT_C,
                    ),
                    &false,
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                }
            };
        });
    }
}
use sp_std::prelude::*;
use sp_std::if_std;
use frame_support::{RuntimeDebugNoBound};
use codec::{Encode, Decode};
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type Domain = String;
type Route = String;
pub enum PathGen {
    /// A value domain/route combination, with domain D separate from route R.
    DomainRoute(String, String),
    /// A path, expressed as <domain> (if route is None) or <domain>:<route>.
    Path(String),
}
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
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Path {
    #[inline]
    fn clone(&self) -> Path {
        match *self {
            Path {
                domain: ref __self_0_0,
                domain_lc: ref __self_0_1,
                route: ref __self_0_2,
                route_lc: ref __self_0_3,
                path: ref __self_0_4,
                path_lc: ref __self_0_5,
            } => Path {
                domain: ::core::clone::Clone::clone(&(*__self_0_0)),
                domain_lc: ::core::clone::Clone::clone(&(*__self_0_1)),
                route: ::core::clone::Clone::clone(&(*__self_0_2)),
                route_lc: ::core::clone::Clone::clone(&(*__self_0_3)),
                path: ::core::clone::Clone::clone(&(*__self_0_4)),
                path_lc: ::core::clone::Clone::clone(&(*__self_0_5)),
            },
        }
    }
}
impl ::core::marker::StructuralEq for Path {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Path {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<Domain>;
            let _: ::core::cmp::AssertParamIsEq<Domain>;
            let _: ::core::cmp::AssertParamIsEq<Route>;
            let _: ::core::cmp::AssertParamIsEq<Domain>;
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
}
impl ::core::marker::StructuralPartialEq for Path {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Path {
    #[inline]
    fn eq(&self, other: &Path) -> bool {
        match *other {
            Path {
                domain: ref __self_1_0,
                domain_lc: ref __self_1_1,
                route: ref __self_1_2,
                route_lc: ref __self_1_3,
                path: ref __self_1_4,
                path_lc: ref __self_1_5,
            } => match *self {
                Path {
                    domain: ref __self_0_0,
                    domain_lc: ref __self_0_1,
                    route: ref __self_0_2,
                    route_lc: ref __self_0_3,
                    path: ref __self_0_4,
                    path_lc: ref __self_0_5,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Path) -> bool {
        match *other {
            Path {
                domain: ref __self_1_0,
                domain_lc: ref __self_1_1,
                route: ref __self_1_2,
                route_lc: ref __self_1_3,
                path: ref __self_1_4,
                path_lc: ref __self_1_5,
            } => match *self {
                Path {
                    domain: ref __self_0_0,
                    domain_lc: ref __self_0_1,
                    route: ref __self_0_2,
                    route_lc: ref __self_0_3,
                    path: ref __self_0_4,
                    path_lc: ref __self_0_5,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                }
            },
        }
    }
}
const _: () = {
    impl core::fmt::Debug for Path {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("Path")
                .field("domain", &self.domain)
                .field("domain_lc", &self.domain_lc)
                .field("route", &self.route)
                .field("route_lc", &self.route_lc)
                .field("path", &self.path)
                .field("path_lc", &self.path_lc)
                .finish()
        }
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for Path {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            _parity_scale_codec::Encode::encode_to(&self.domain, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.domain_lc, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.route, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.route_lc, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.path, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.path_lc, __codec_dest_edqy);
        }
    }
    impl _parity_scale_codec::EncodeLike for Path {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for Path {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(Path {
                domain: {
                    let __codec_res_edqy =
                        <Domain as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Path::domain`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                domain_lc: {
                    let __codec_res_edqy =
                        <Domain as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Path::domain_lc`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                route: {
                    let __codec_res_edqy =
                        <Route as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Path::route`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                route_lc: {
                    let __codec_res_edqy =
                        <Domain as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Path::route_lc`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                path: {
                    let __codec_res_edqy =
                        <String as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Path::path`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                path_lc: {
                    let __codec_res_edqy =
                        <String as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Path::path_lc`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
            })
        }
    }
};
/// Registered path.
pub struct RegisteredPath<T: frame_system::Config> {
    /// Owner of the path.
    pub owner: AccountIdOf<T>,
    /// Full path (i.e., <domain>:<route>).  The path for a domain-name reservation is <domain>:
    pub path: Path,
    /// Optional: hash for the code referenced by this contract.
    pub code_hash: Option<CodeHashOf<T>>,
    /// Optional: code for this contract.
    pub code: Option<Code>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + frame_system::Config> ::core::clone::Clone for RegisteredPath<T> {
    #[inline]
    fn clone(&self) -> RegisteredPath<T> {
        match *self {
            RegisteredPath {
                owner: ref __self_0_0,
                path: ref __self_0_1,
                code_hash: ref __self_0_2,
                code: ref __self_0_3,
            } => RegisteredPath {
                owner: ::core::clone::Clone::clone(&(*__self_0_0)),
                path: ::core::clone::Clone::clone(&(*__self_0_1)),
                code_hash: ::core::clone::Clone::clone(&(*__self_0_2)),
                code: ::core::clone::Clone::clone(&(*__self_0_3)),
            },
        }
    }
}
impl<T: frame_system::Config> ::core::marker::StructuralEq for RegisteredPath<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + frame_system::Config> ::core::cmp::Eq for RegisteredPath<T> {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountIdOf<T>>;
            let _: ::core::cmp::AssertParamIsEq<Path>;
            let _: ::core::cmp::AssertParamIsEq<Option<CodeHashOf<T>>>;
            let _: ::core::cmp::AssertParamIsEq<Option<Code>>;
        }
    }
}
impl<T: frame_system::Config> ::core::marker::StructuralPartialEq for RegisteredPath<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + frame_system::Config> ::core::cmp::PartialEq
    for RegisteredPath<T>
{
    #[inline]
    fn eq(&self, other: &RegisteredPath<T>) -> bool {
        match *other {
            RegisteredPath {
                owner: ref __self_1_0,
                path: ref __self_1_1,
                code_hash: ref __self_1_2,
                code: ref __self_1_3,
            } => match *self {
                RegisteredPath {
                    owner: ref __self_0_0,
                    path: ref __self_0_1,
                    code_hash: ref __self_0_2,
                    code: ref __self_0_3,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &RegisteredPath<T>) -> bool {
        match *other {
            RegisteredPath {
                owner: ref __self_1_0,
                path: ref __self_1_1,
                code_hash: ref __self_1_2,
                code: ref __self_1_3,
            } => match *self {
                RegisteredPath {
                    owner: ref __self_0_0,
                    path: ref __self_0_1,
                    code_hash: ref __self_0_2,
                    code: ref __self_0_3,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                }
            },
        }
    }
}
const _: () = {
    impl<T: frame_system::Config> core::fmt::Debug for RegisteredPath<T> {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_struct("RegisteredPath")
                .field("owner", &self.owner)
                .field("path", &self.path)
                .field("code_hash", &self.code_hash)
                .field("code", &self.code)
                .finish()
        }
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: frame_system::Config> _parity_scale_codec::Encode for RegisteredPath<T>
    where
        AccountIdOf<T>: _parity_scale_codec::Encode,
        AccountIdOf<T>: _parity_scale_codec::Encode,
        Option<CodeHashOf<T>>: _parity_scale_codec::Encode,
        Option<CodeHashOf<T>>: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            _parity_scale_codec::Encode::encode_to(&self.owner, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.path, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.code_hash, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.code, __codec_dest_edqy);
        }
    }
    impl<T: frame_system::Config> _parity_scale_codec::EncodeLike for RegisteredPath<T>
    where
        AccountIdOf<T>: _parity_scale_codec::Encode,
        AccountIdOf<T>: _parity_scale_codec::Encode,
        Option<CodeHashOf<T>>: _parity_scale_codec::Encode,
        Option<CodeHashOf<T>>: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: frame_system::Config> _parity_scale_codec::Decode for RegisteredPath<T>
    where
        AccountIdOf<T>: _parity_scale_codec::Decode,
        AccountIdOf<T>: _parity_scale_codec::Decode,
        Option<CodeHashOf<T>>: _parity_scale_codec::Decode,
        Option<CodeHashOf<T>>: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(RegisteredPath::<T> {
                owner: {
                    let __codec_res_edqy =
                        <AccountIdOf<T> as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `RegisteredPath::owner`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                path: {
                    let __codec_res_edqy =
                        <Path as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `RegisteredPath::path`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                code_hash: {
                    let __codec_res_edqy =
                        <Option<CodeHashOf<T>> as _parity_scale_codec::Decode>::decode(
                            __codec_input_edqy,
                        );
                    match __codec_res_edqy {
                        Err(e) => {
                            return Err(e.chain("Could not decode `RegisteredPath::code_hash`"))
                        }
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                code: {
                    let __codec_res_edqy =
                        <Option<Code> as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `RegisteredPath::code`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
            })
        }
    }
};
/// Create a new Path object, with any ":" removed.
pub fn new_path(path_gen: PathGen) -> Result<Path, String> {
    let path_result = match path_gen {
        PathGen::DomainRoute(d, r) => Ok((d.clone().replace(":", ""), r.clone())),
        PathGen::Path(p) => {
            let split_path: Vec<&str> = p.split(":").collect();
            if (split_path.len() == 1) || (split_path.len() == 2) {
                let route: String = if split_path.len() == 1 {
                    String::from("")
                } else {
                    String::from(split_path[1].clone())
                };
                Ok((String::from(split_path[0].clone()), route))
            } else {
                Err("Malformed Path")
            }
        }
    }?;
    let (domain, route) = path_result;
    let domain_lc: String = domain.clone().to_lowercase();
    let route_lc: String = route.clone().to_lowercase();
    let path: String = {
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["", ":"],
            &match (&domain.clone(), &route) {
                (arg0, arg1) => [
                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                ],
            },
        ));
        res
    };
    let path_lc: String = path.clone().to_lowercase();
    Ok(Path {
        domain,
        route,
        domain_lc,
        route_lc,
        path,
        path_lc,
    })
}
pub use pallet::*;
///
///			The module that hosts all the
///			[FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame)
///			types needed to add this pallet to a
///			[runtime](https://substrate.dev/docs/en/knowledgebase/runtime/).
///
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use super::*;
    ///
    ///			Configuration trait of this pallet.
    ///
    ///			Implement this type for a runtime in order to customize this pallet.
    ///
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Maximum size for a domain name, in bytes
        type MaxDomainByteSize: Get<u32>;
        /// Maximum size for a route name, in bytes
        type MaxRouteByteSize: Get<u32>;
    }
    ///
    ///			The [pallet](https://substrate.dev/docs/en/knowledgebase/runtime/pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(PhantomData<T>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    /// A table of registered paths (i.e., domain/route combinations).  A domain itself will always be registered with a route of None;
    /// all other registered paths will have a Some<Vec<u8>> route string
    #[allow(type_alias_bounds)]
    pub type RegisteredPaths<T: Config> = StorageDoubleMap<
        _GeneratedPrefixForStorageRegisteredPaths<T>,
        Blake2_128Concat,
        Domain,
        Blake2_128Concat,
        Route,
        RegisteredPath<T>,
    >;
    /// A table of registered approvers for a given path, i.e., account(s) (if any) that have been approved by the path owner to also manage the path
    #[allow(type_alias_bounds)]
    pub type RegisteredApprovers<T: Config> = StorageDoubleMap<
        _GeneratedPrefixForStorageRegisteredApprovers<T>,
        Blake2_128Concat,
        Domain,
        Blake2_128Concat,
        (Route, Approver<T>),
        (),
    >;
    ///
    ///			The [event](https://substrate.dev/docs/en/knowledgebase/runtime/events) emitted
    ///			by this pallet.
    ///
    pub enum Event<T: Config> {
        /// A name was set or reset (which will remove all judgements). \[who\]
        OwnerSet(T::AccountId, Domain),
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::OwnerSet(ref _0, ref _1) => {
                        Self::OwnerSet(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::OwnerSet(_0, _1), Self::OwnerSet(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::OwnerSet { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::OwnerSet { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::OwnerSet(ref _0, ref _1) => fmt
                        .debug_tuple("Event::OwnerSet")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Event::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Encode for Event<T>
        where
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::OwnerSet(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(0usize as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> _parity_scale_codec::EncodeLike for Event<T>
        where
            T::AccountId: _parity_scale_codec::Encode,
            T::AccountId: _parity_scale_codec::Encode,
        {
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Decode for Event<T>
        where
            T::AccountId: _parity_scale_codec::Decode,
            T::AccountId: _parity_scale_codec::Decode,
        {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(Event::<T>::OwnerSet(
                        {
                            let __codec_res_edqy =
                                <T::AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `Event::OwnerSet.0`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <Domain as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `Event::OwnerSet.1`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    )),
                    _ => Err("Could not decode `Event`, variant doesn\'t exist".into()),
                }
            }
        }
    };
    ///
    ///			Custom [dispatch errors](https://substrate.dev/docs/en/knowledgebase/runtime/errors)
    ///			of this pallet.
    ///
    pub enum Error<T> {
        #[doc(hidden)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T)>,
            frame_support::Never,
        ),
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
        pub fn get_registered_path_info(
            domain: String,
            route: String,
        ) -> Result<Option<RegisteredPath<T>>, Error<T>> {
            match new_path(PathGen::DomainRoute(domain, route)) {
                Ok(path) => Ok(RegisteredPaths::<T>::get(path.domain_lc, path.route_lc)),
                Err(_) => Err(Error::<T>::MalformedPath)?,
            }
        }
        /// Return true if registered, false if not (Malformed domains/routes are reported as false)
        pub fn is_registered_path(domain: String, route: String) -> bool {
            match Pallet::<T>::get_registered_path_info(domain, route) {
                Ok(Some(_)) => true,
                _ => false,
            }
        }
        /// Return Some(registered path) if registered, None if not registered, or MalformedPath if the given domain and route are invalid
        pub fn get_registered_approver_info(
            domain: String,
            route: String,
            approver: Approver<T>,
        ) -> Result<Option<()>, Error<T>> {
            match new_path(PathGen::DomainRoute(domain, route)) {
                Ok(path) => Ok(RegisteredApprovers::<T>::get(
                    path.domain_lc,
                    (path.route_lc, approver),
                )),
                Err(_) => Err(Error::<T>::MalformedPath)?,
            }
        }
        /// Return true if registered, false if not (Malformed domains/routes are reported as false)
        pub fn is_registered_approver(
            domain: String,
            route: String,
            approver: Approver<T>,
        ) -> bool {
            match Pallet::<T>::get_registered_approver_info(domain, route, approver) {
                Ok(Some(_)) => true,
                _ => false,
            }
        }
    }
    impl<T: Config> Pallet<T> {
        /// Register a domain.
        /// A domain must be registered before routes on that domain can be created.  register_domain will lower-case the string before
        pub fn register_domain(
            origin: OriginFor<T>,
            domain: String,
            code_hash: Option<CodeHashOf<T>>,
            code: Option<Code>,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;
            let null_route = String::from("");
            match new_path(PathGen::DomainRoute(domain, null_route)) {
                Ok(path) => match Pallet::<T>::is_registered_path(
                    path.domain_lc.clone(),
                    path.route_lc.clone(),
                ) {
                    false => {
                        let registered_path = RegisteredPath::<T> {
                            owner: owner.clone(),
                            path: path.clone(),
                            code_hash: code_hash.clone(),
                            code: code.clone(),
                        };
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                                &["in register_domain, Ok path, registered_path = ", "\n"],
                                &match (&registered_path,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Debug::fmt,
                                    )],
                                },
                                &[::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                            ));
                        };
                        {
                            :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1_formatted (& ["in register_domain, Adding the following K/K to RegisteredPaths = " , ",  " , "\n"] , & match (& registered_path . path . domain_lc . clone () , & registered_path . clone () . path . route_lc . clone ()) { (arg0 , arg1) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Debug :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg1 , :: core :: fmt :: Debug :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }])) ;
                        };
                        RegisteredPaths::<T>::insert(
                            registered_path.path.domain_lc.clone(),
                            registered_path.clone().path.route_lc.clone(),
                            registered_path.clone(),
                        );
                        {
                            :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1_formatted (& ["in register_domain, Adding the following K/K to RegisteredApprovers = " , ",  " , "\n"] , & match (& registered_path . clone () . path . domain_lc . clone () , & (registered_path . clone () . path . route_lc . clone () , owner . clone ())) { (arg0 , arg1) => [:: core :: fmt :: ArgumentV1 :: new (arg0 , :: core :: fmt :: Debug :: fmt) , :: core :: fmt :: ArgumentV1 :: new (arg1 , :: core :: fmt :: Debug :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }])) ;
                        };
                        RegisteredApprovers::<T>::insert(
                            registered_path.clone().path.domain_lc.clone(),
                            (registered_path.clone().path.route_lc.clone(), owner.clone()),
                            (),
                        );
                        Self::deposit_event(Event::OwnerSet(
                            owner.clone(),
                            registered_path.clone().path.domain_lc.clone(),
                        ));
                        Ok(Some(10_000).into())
                    }
                    true => Err(Error::<T>::DomainAlreadyExists)?,
                },
                Err(_) => Err(Error::<T>::MalformedDomain)?,
            }
        }
        pub fn add_approver(
            origin: OriginFor<T>,
            path: String,
            approver: Approver<T>,
        ) -> DispatchResultWithPostInfo {
            let source = ensure_signed(origin)?;
            let path_lc = path.clone().to_lowercase();
            let split_path_lc: Vec<&str> = path_lc.split(":").collect();
            if (split_path_lc.len() == 1)
                || ((split_path_lc.len() == 2) && (split_path_lc[1].len() == 0))
            {
                let domain_final: Domain = split_path_lc[0].clone().into();
                let route_final: Route = if split_path_lc.len() == 1 {
                    String::from("")
                } else {
                    String::from(split_path_lc[1].clone())
                };
                {
                    if !RegisteredPaths::<T>::contains_key(&domain_final, route_final.clone()) {
                        {
                            return Err(Error::<T>::DomainNotRegistered.into());
                        };
                    }
                };
                if RegisteredApprovers::<T>::contains_key(
                    domain_final.clone(),
                    (route_final.clone(), source),
                ) {
                    RegisteredApprovers::<T>::insert(
                        domain_final.clone(),
                        (route_final.clone(), approver),
                        (),
                    );
                    Ok(Some(10_000).into())
                } else {
                    Err(Error::<T>::CallerNotApproved)?
                }
            } else {
                Err(Error::<T>::MalformedPath)?
            }
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn module_constants_metadata(
        ) -> &'static [frame_support::dispatch::ModuleConstantMetadata] {
            &[
                {
                    #[allow(non_upper_case_types)]
                    #[allow(non_camel_case_types)]
                    struct MaxDomainByteSizeDefaultByteGetter<T>(
                        frame_support::sp_std::marker::PhantomData<(T)>,
                    );
                    impl<T: Config> frame_support::dispatch::DefaultByte for MaxDomainByteSizeDefaultByteGetter<T> {
                        fn default_byte(&self) -> frame_support::sp_std::vec::Vec<u8> {
                            let value =
                                <T::MaxDomainByteSize as frame_support::traits::Get<u32>>::get();
                            frame_support::codec::Encode::encode(&value)
                        }
                    }
                    unsafe impl<T: Config> Send for MaxDomainByteSizeDefaultByteGetter<T> {}
                    unsafe impl<T: Config> Sync for MaxDomainByteSizeDefaultByteGetter<T> {}
                    frame_support::dispatch::ModuleConstantMetadata {
                        name: frame_support::dispatch::DecodeDifferent::Encode("MaxDomainByteSize"),
                        ty: frame_support::dispatch::DecodeDifferent::Encode("u32"),
                        value: frame_support::dispatch::DecodeDifferent::Encode(
                            frame_support::dispatch::DefaultByteGetter(
                                &MaxDomainByteSizeDefaultByteGetter::<T>(
                                    frame_support::sp_std::marker::PhantomData,
                                ),
                            ),
                        ),
                        documentation: frame_support::dispatch::DecodeDifferent::Encode(&[
                            " Maximum size for a domain name, in bytes",
                        ]),
                    }
                },
                {
                    #[allow(non_upper_case_types)]
                    #[allow(non_camel_case_types)]
                    struct MaxRouteByteSizeDefaultByteGetter<T>(
                        frame_support::sp_std::marker::PhantomData<(T)>,
                    );
                    impl<T: Config> frame_support::dispatch::DefaultByte for MaxRouteByteSizeDefaultByteGetter<T> {
                        fn default_byte(&self) -> frame_support::sp_std::vec::Vec<u8> {
                            let value =
                                <T::MaxRouteByteSize as frame_support::traits::Get<u32>>::get();
                            frame_support::codec::Encode::encode(&value)
                        }
                    }
                    unsafe impl<T: Config> Send for MaxRouteByteSizeDefaultByteGetter<T> {}
                    unsafe impl<T: Config> Sync for MaxRouteByteSizeDefaultByteGetter<T> {}
                    frame_support::dispatch::ModuleConstantMetadata {
                        name: frame_support::dispatch::DecodeDifferent::Encode("MaxRouteByteSize"),
                        ty: frame_support::dispatch::DecodeDifferent::Encode("u32"),
                        value: frame_support::dispatch::DecodeDifferent::Encode(
                            frame_support::dispatch::DefaultByteGetter(
                                &MaxRouteByteSizeDefaultByteGetter::<T>(
                                    frame_support::sp_std::marker::PhantomData,
                                ),
                            ),
                        ),
                        documentation: frame_support::dispatch::DecodeDifferent::Encode(&[
                            " Maximum size for a route name, in bytes",
                        ]),
                    }
                },
            ]
        }
    }
    impl<T: Config> frame_support::error::ModuleErrorMetadata for Pallet<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            <Error<T> as frame_support::error::ModuleErrorMetadata>::metadata()
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetPalletVersion for Pallet<T> {
        fn current_version() -> frame_support::traits::PalletVersion {
            frame_support::traits::PalletVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
        fn storage_version() -> Option<frame_support::traits::PalletVersion> {
            let key = frame_support::traits::PalletVersion::storage_key::<
                <T as frame_system::Config>::PalletInfo,
                Self,
            >()
            .expect("Every active pallet has a name in the runtime; qed");
            frame_support::storage::unhashed::get(&key)
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            frame_support::traits::PalletVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
            .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        /// Register a domain.
        /// A domain must be registered before routes on that domain can be created.  register_domain will lower-case the string before
        register_domain(String, Option<CodeHashOf<T>>, Option<Code>),
        add_approver(String, Approver<T>),
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::register_domain(ref _0, ref _1, ref _2) => fmt
                        .debug_tuple("Call::register_domain")
                        .field(&_0)
                        .field(&_1)
                        .field(&_2)
                        .finish(),
                    Self::add_approver(ref _0, ref _1) => fmt
                        .debug_tuple("Call::add_approver")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::register_domain(ref _0, ref _1, ref _2) => Self::register_domain(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                        core::clone::Clone::clone(_2),
                    ),
                    Self::add_approver(ref _0, ref _1) => Self::add_approver(
                        core::clone::Clone::clone(_0),
                        core::clone::Clone::clone(_1),
                    ),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (
                        Self::register_domain(_0, _1, _2),
                        Self::register_domain(_0_other, _1_other, _2_other),
                    ) => true && _0 == _0_other && _1 == _1_other && _2 == _2_other,
                    (Self::add_approver(_0, _1), Self::add_approver(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::__Ignore { .. }, Self::register_domain { .. }) => false,
                    (Self::__Ignore { .. }, Self::add_approver { .. }) => false,
                    (Self::register_domain { .. }, Self::__Ignore { .. }) => false,
                    (Self::register_domain { .. }, Self::add_approver { .. }) => false,
                    (Self::add_approver { .. }, Self::__Ignore { .. }) => false,
                    (Self::add_approver { .. }, Self::register_domain { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::register_domain(ref aa, ref ba, ref ca) => {
                        __codec_dest_edqy.push_byte(0usize as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ca, __codec_dest_edqy);
                    }
                    Call::add_approver(ref aa, ref ba) => {
                        __codec_dest_edqy.push_byte(1usize as u8);
                        _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        impl<T: Config> _parity_scale_codec::EncodeLike for Call<T> {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Config> _parity_scale_codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0usize as u8 => {
                        Ok(Call::<T>::register_domain(
                            {
                                let __codec_res_edqy =
                                    <String as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    Err(e) => {
                                        return Err(
                                            e.chain("Could not decode `Call::register_domain.0`")
                                        )
                                    }
                                    Ok(__codec_res_edqy) => __codec_res_edqy,
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Option<CodeHashOf<T>> as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    Err(e) => {
                                        return Err(
                                            e.chain("Could not decode `Call::register_domain.1`")
                                        )
                                    }
                                    Ok(__codec_res_edqy) => __codec_res_edqy,
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Option<Code> as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    Err(e) => {
                                        return Err(
                                            e.chain("Could not decode `Call::register_domain.2`")
                                        )
                                    }
                                    Ok(__codec_res_edqy) => __codec_res_edqy,
                                }
                            },
                        ))
                    }
                    __codec_x_edqy if __codec_x_edqy == 1usize as u8 => {
                        Ok(Call::<T>::add_approver(
                            {
                                let __codec_res_edqy =
                                    <String as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    Err(e) => {
                                        return Err(
                                            e.chain("Could not decode `Call::add_approver.0`")
                                        )
                                    }
                                    Ok(__codec_res_edqy) => __codec_res_edqy,
                                }
                            },
                            {
                                let __codec_res_edqy =
                                    <Approver<T> as _parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    Err(e) => {
                                        return Err(
                                            e.chain("Could not decode `Call::add_approver.1`")
                                        )
                                    }
                                    Ok(__codec_res_edqy) => __codec_res_edqy,
                                }
                            },
                        ))
                    }
                    _ => Err("Could not decode `Call`, variant doesn\'t exist".into()),
                }
            }
        }
    };
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::register_domain(ref domain, ref code_hash, ref code) => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &String,
                        &Option<CodeHashOf<T>>,
                        &Option<Code>,
                    )>>::weigh_data(
                        &__pallet_base_weight, (domain, code_hash, code)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &String,
                        &Option<CodeHashOf<T>>,
                        &Option<Code>,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (domain, code_hash, code)
                    );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<(
                        &String,
                        &Option<CodeHashOf<T>>,
                        &Option<Code>,
                    )>>::pays_fee(
                        &__pallet_base_weight, (domain, code_hash, code)
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::add_approver(ref path, ref approver) => {
                    let __pallet_base_weight = 10_000;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &String,
                        &Approver<T>,
                    )>>::weigh_data(
                        &__pallet_base_weight, (path, approver)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &String,
                        &Approver<T>,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (path, approver)
                    );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&String, &Approver<T>)>>::pays_fee(
                            &__pallet_base_weight,
                            (path, approver),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__Ignore cannot be used",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::register_domain(..) => "register_domain",
                Self::add_approver(..) => "add_approver",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem cannot be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["register_domain", "add_approver"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::register_domain(domain, code_hash, code) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "register_domain",
                                    "compose_register::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("compose/register/src/lib.rs"),
                                    Some(135u32),
                                    Some("compose_register::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::register_domain(origin, domain, code_hash, code)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::add_approver(path, approver) => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "add_approver",
                                    "compose_register::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("compose/register/src/lib.rs"),
                                    Some(135u32),
                                    Some("compose_register::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && CALLSITE.is_enabled(interest)
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span = CALLSITE.disabled_span();
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    <Pallet<T>>::add_approver(origin, path, approver)
                        .map(Into::into)
                        .map_err(Into::into)
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    {
                        {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                &match (&"__PhantomItem cannot be used.",) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                },
                            ))
                        }
                    };
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        #[allow(dead_code)]
        pub fn call_functions() -> &'static [frame_support::dispatch::FunctionMetadata] {
            & [frame_support :: dispatch :: FunctionMetadata { name : frame_support :: dispatch :: DecodeDifferent :: Encode ("register_domain") , arguments : frame_support :: dispatch :: DecodeDifferent :: Encode (& [frame_support :: dispatch :: FunctionArgumentMetadata { name : frame_support :: dispatch :: DecodeDifferent :: Encode ("domain") , ty : frame_support :: dispatch :: DecodeDifferent :: Encode ("String") , } , frame_support :: dispatch :: FunctionArgumentMetadata { name : frame_support :: dispatch :: DecodeDifferent :: Encode ("code_hash") , ty : frame_support :: dispatch :: DecodeDifferent :: Encode ("Option<CodeHashOf<T>>") , } , frame_support :: dispatch :: FunctionArgumentMetadata { name : frame_support :: dispatch :: DecodeDifferent :: Encode ("code") , ty : frame_support :: dispatch :: DecodeDifferent :: Encode ("Option<Code>") , }]) , documentation : frame_support :: dispatch :: DecodeDifferent :: Encode (& [" Register a domain." , " A domain must be registered before routes on that domain can be created.  register_domain will lower-case the string before"]) , } , frame_support :: dispatch :: FunctionMetadata { name : frame_support :: dispatch :: DecodeDifferent :: Encode ("add_approver") , arguments : frame_support :: dispatch :: DecodeDifferent :: Encode (& [frame_support :: dispatch :: FunctionArgumentMetadata { name : frame_support :: dispatch :: DecodeDifferent :: Encode ("path") , ty : frame_support :: dispatch :: DecodeDifferent :: Encode ("String") , } , frame_support :: dispatch :: FunctionArgumentMetadata { name : frame_support :: dispatch :: DecodeDifferent :: Encode ("approver") , ty : frame_support :: dispatch :: DecodeDifferent :: Encode ("Approver<T>") , }]) , documentation : frame_support :: dispatch :: DecodeDifferent :: Encode (& []) , }]
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        pub fn as_u8(&self) -> u8 {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"`__Ignore` can never be constructed",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                Self::DomainAlreadyExists => 0usize as u8,
                Self::PathAlreadyExists => 1usize as u8,
                Self::DomainNameTooLarge => 2usize as u8,
                Self::DomainNotRegistered => 3usize as u8,
                Self::RouteNotRegistered => 4usize as u8,
                Self::CallerNotApproved => 5usize as u8,
                Self::MalformedPath => 6usize as u8,
                Self::MalformedDomain => 7usize as u8,
            }
        }
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"`__Ignore` can never be constructed",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
                Self::DomainAlreadyExists => "DomainAlreadyExists",
                Self::PathAlreadyExists => "PathAlreadyExists",
                Self::DomainNameTooLarge => "DomainNameTooLarge",
                Self::DomainNotRegistered => "DomainNotRegistered",
                Self::RouteNotRegistered => "RouteNotRegistered",
                Self::CallerNotApproved => "CallerNotApproved",
                Self::MalformedPath => "MalformedPath",
                Self::MalformedDomain => "MalformedDomain",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            let index = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: index :: < Pallet < T > > () . expect ("Every active module has an index in the runtime; qed") as u8 ;
            frame_support::sp_runtime::DispatchError::Module {
                index,
                error: err.as_u8(),
                message: Some(err.as_str()),
            }
        }
    }
    impl<T: Config> frame_support::error::ModuleErrorMetadata for Error<T> {
        fn metadata() -> &'static [frame_support::error::ErrorMetadata] {
            & [frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("DomainAlreadyExists") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" Domain already exists."]) , } , frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("PathAlreadyExists") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" Domain/Route Combination already exists."]) , } , frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("DomainNameTooLarge") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" Domain name is too large."]) , } , frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("DomainNotRegistered") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" Domain does not exist."]) , } , frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("RouteNotRegistered") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" Domain exists but route is not registered on the domain."]) , } , frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("CallerNotApproved") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" The source of a call is not authorized to perform the call."]) , } , frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("MalformedPath") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" The path does not follow the <domain>:<slash-separated route> convention."]) , } , frame_support :: error :: ErrorMetadata { name : frame_support :: error :: DecodeDifferent :: Encode ("MalformedDomain") , documentation : frame_support :: error :: DecodeDifferent :: Encode (& [" The domain does not follow the <domain>:<slash-separated route> convention."]) , }]
        }
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::Event as From<Event<T>>>::from(event);
            let event =
                <<T as Config>::Event as Into<<T as frame_system::Config>::Event>>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Event<T> {
        #[allow(dead_code)]
        #[doc(hidden)]
        pub fn metadata() -> &'static [frame_support::event::EventMetadata] {
            &[frame_support::event::EventMetadata {
                name: frame_support::event::DecodeDifferent::Encode("OwnerSet"),
                arguments: frame_support::event::DecodeDifferent::Encode(&["AccountId", "Domain"]),
                documentation: frame_support::event::DecodeDifferent::Encode(&[
                    " A name was set or reset (which will remove all judgements). \\[who\\]",
                ]),
            }]
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::StorageMetadata {
            frame_support :: metadata :: StorageMetadata { prefix : frame_support :: metadata :: DecodeDifferent :: Encode (< < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed")) , entries : frame_support :: metadata :: DecodeDifferent :: Encode (& [frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< RegisteredPaths < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: NAME) , modifier : < RegisteredPaths < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: DoubleMap { hasher : < RegisteredPaths < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: HASHER1 , key2_hasher : < RegisteredPaths < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: HASHER2 , key1 : frame_support :: metadata :: DecodeDifferent :: Encode ("Domain") , key2 : frame_support :: metadata :: DecodeDifferent :: Encode ("Route") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("RegisteredPath<T>") , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< RegisteredPaths < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& [" A table of registered paths (i.e., domain/route combinations).  A domain itself will always be registered with a route of None; " , " all other registered paths will have a Some<Vec<u8>> route string"]) , } , frame_support :: metadata :: StorageEntryMetadata { name : frame_support :: metadata :: DecodeDifferent :: Encode (< RegisteredApprovers < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: NAME) , modifier : < RegisteredApprovers < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: MODIFIER , ty : frame_support :: metadata :: StorageEntryType :: DoubleMap { hasher : < RegisteredApprovers < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: HASHER1 , key2_hasher : < RegisteredApprovers < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: HASHER2 , key1 : frame_support :: metadata :: DecodeDifferent :: Encode ("Domain") , key2 : frame_support :: metadata :: DecodeDifferent :: Encode ("(Route, Approver<T>)") , value : frame_support :: metadata :: DecodeDifferent :: Encode ("()") , } , default : frame_support :: metadata :: DecodeDifferent :: Encode (< RegisteredApprovers < T > as frame_support :: storage :: types :: StorageDoubleMapMetadata > :: DEFAULT) , documentation : frame_support :: metadata :: DecodeDifferent :: Encode (& [" A table of registered approvers for a given path, i.e., account(s) (if any) that have been approved by the path owner to also manage the path"]) , }]) , }
        }
    }
    pub struct _GeneratedPrefixForStorageRegisteredPaths<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageRegisteredPaths<T>
    {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "RegisteredPaths";
    }
    pub struct _GeneratedPrefixForStorageRegisteredApprovers<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
        for _GeneratedPrefixForStorageRegisteredApprovers<T>
    {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
            .expect("Every active pallet has a name in the runtime; qed")
        }
        const STORAGE_PREFIX: &'static str = "RegisteredApprovers";
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    pub(super) trait Store {
        type RegisteredPaths;
        type RegisteredApprovers;
    }
    impl<T: Config> Store for Pallet<T> {
        type RegisteredPaths = RegisteredPaths<T>;
        type RegisteredApprovers = RegisteredApprovers<T>;
    }
    impl<T: Config> frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
    }
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "compose_register::pallet",
                            ::tracing::Level::TRACE,
                            Some("compose/register/src/lib.rs"),
                            Some(135u32),
                            Some("compose_register::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "compose_register::pallet",
                            ::tracing::Level::TRACE,
                            Some("compose/register/src/lib.rs"),
                            Some(135u32),
                            Some("compose_register::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "compose_register::pallet",
                            ::tracing::Level::TRACE,
                            Some("compose/register/src/lib.rs"),
                            Some(135u32),
                            Some("compose_register::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let new_storage_version = frame_support::traits::PalletVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            };
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "\u{2705} no migration for ",
                                ", setting storage version to ",
                            ],
                            &match (&pallet_name, &new_storage_version) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "compose_register::pallet",
                            "compose/register/src/lib.rs",
                            135u32,
                        ),
                    );
                }
            };
            let result = <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_runtime_upgrade();
            new_storage_version.put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
            let additional_write =
                <<T as frame_system::Config>::DbWeight as frame_support::traits::Get<_>>::get()
                    .writes(1);
            result.saturating_add(additional_write)
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &runtime_integrity_tests,
        &successful_register,
        &successful_approval,
    ])
}
