// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use codec::{Encode, Decode, EncodeLike};
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

/// A serializable string equivalent
#[derive(Eq, RuntimeDebug, Clone)]
pub struct SerString {
	val: Vec<u8>,
}

impl SerString {
    pub fn from_str(s: &str) -> Self {
        Self { val: s.into() }
    }

    // pub fn from_string(s: String) -> Self {
    //     Self { val: s.into() }
    // }

    pub fn to_str(&self) -> &str {
        // See this website for various conversions to/from String, str, Vec<u8>, Vec<char>:  https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424
        sp_std::str::from_utf8(&self.val).unwrap()
    }

    // pub fn to_string(&self) -> String {
    //     // See this website for various conversions to/from String, str, Vec<u8>, Vec<char>:  https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424
    //     sp_std::str::from_utf8(&self.val).unwrap().into()
    // }

	// pub fn split(&self, split_str: &str) -> sp_std::vec::IntoIter<SerString> {
	// 	self.to_str().split(split_str).map(|s| Self::from_str(s)).collect::<Vec<SerString>>().into_iter()
	// }

	pub fn replace(&self, from: &str, to: &str) -> SerString {
		SerString::from_str(&self.to_str().replace(from, to)[..])
	}

	pub fn to_lowercase(&self) -> SerString {
		SerString::from_str(&self.to_str().to_lowercase()[..])
	}

	pub fn split(&self, separator: &str) -> Vec<SerString> {
		self.to_str().split(separator).map(SerString::from_str).collect()
	}

	pub fn join(v: Vec<SerString>, separator: &str) -> SerString {
		Self::from_str(&v.iter().map(|s| s.to_str()).collect::<Vec<&str>>().join(separator)[..])
		// self.to_str().split(separator).map(SerString::from_str).collect()
	}
}
/// Convenience macro to use the format! interface to get a SerString
#[macro_export]
macro_rules! format_ser_string {
	($($args:tt)*) => {{
        SerString { val: sp_std::fmt::format!($($args)*).as_bytes().to_vec() }
	}};
}


impl From<&str> for SerString {
	fn from(s: &str) -> Self {
		SerString::from_str(s)
	}
}

impl Default for SerString {
	fn default() -> Self {
		Self { val: Default::default() }
	}
}

impl PartialEq for SerString {
	fn eq(&self, other: &Self) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl AsRef<[u8]> for SerString {
	fn as_ref(&self) -> &[u8] {
		self.val.as_ref()
	}
}

impl Encode for SerString {
	fn encode(&self) -> Vec<u8> {
        self.val.encode()
	}
}

impl EncodeLike for SerString { }

impl Decode for SerString {
	fn decode<I: codec::Input>(value: &mut I) -> Result<Self, codec::Error> {
		Decode::decode(value).map(|val| {Self { val }})
	}
}

// #[cfg(feature = "std")]
impl sp_std::fmt::Display for SerString {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        write!(f, "{}", self.to_str())
	}
}

#[cfg(feature = "std")]
impl serde::Serialize for SerString {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.val.serialize(serializer)
	}
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for SerString {
	fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
		String::deserialize(de).map(|val| { Self { val: val.into() }})
	}
}

/// Create a const [`SerString`].
#[macro_export]
macro_rules! create_compose_str {
	( $y:expr ) => {{ $crate::SerString::Owned($y) }}
}