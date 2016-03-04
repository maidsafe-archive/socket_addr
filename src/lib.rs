// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

//! A very simple ScoketAddr structure that is serialisable and makes use of the `ip` crate.
//! There is no intent here to provide many more traits (perhaps serde).
//! Deref is implemented to allow easy access to the underlying `net::ScoketAddr`

#![doc(html_logo_url =
           "https://raw.githubusercontent.com/maidsafe/QA/master/Images/maidsafe_logo.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
       html_root_url = "http://maidsafe.github.io/socket_addr")]

// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, drop_with_repr_extern, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unsafe_code, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations, variant_size_differences)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature="clippy", allow(use_debug))]

extern crate rustc_serialize;

use std::net;
use std::net::IpAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::fmt;
use rustc_serialize::{Encodable, Decodable, Encoder, Decoder};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
/// Wrapper around `std::net::SocketAddr` to enable it to encoded and decoded.
pub struct SocketAddr(pub net::SocketAddr);

impl Deref for SocketAddr {
    type Target = net::SocketAddr;

    fn deref(&self) -> &net::SocketAddr {
        &self.0
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Encodable for SocketAddr {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let as_string = format!("{}", self.0);
        s.emit_str(&as_string[..])
    }
}

impl Decodable for SocketAddr {
    fn decode<D: Decoder>(d: &mut D) -> Result<SocketAddr, D::Error> {
        let as_string = try!(d.read_str());
        match net::SocketAddr::from_str(&as_string[..]) {
            Ok(sa) => Ok(SocketAddr(sa)),
            Err(e) => {
                let err = format!("Failed to decode SocketAddr: {}", e);
                Err(d.error(&err[..]))
            }
        }
    }
}

impl SocketAddr {
    /// Construct new from ip::IpAddr and port
    pub fn new(ip: IpAddr, port: u16) -> Self {
        SocketAddr(net::SocketAddr::new(ip, port))
    }
}

/// Utility struct of SocketAddrV4 for hole punching
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct SocketAddrV4(pub net::SocketAddrV4);

impl Deref for SocketAddrV4 {
    type Target = net::SocketAddrV4;

    fn deref(&self) -> &net::SocketAddrV4 {
        &self.0
    }
}

impl Encodable for SocketAddrV4 {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let as_string = format!("{}", self.0);
        s.emit_str(&as_string[..])
    }
}

impl Decodable for SocketAddrV4 {
    fn decode<D: Decoder>(d: &mut D) -> Result<SocketAddrV4, D::Error> {
        let as_string = try!(d.read_str());
        match net::SocketAddrV4::from_str(&as_string[..]) {
            Ok(sa) => Ok(SocketAddrV4(sa)),
            Err(e) => {
                let err = format!("Failed to decode SocketAddrV4: {}", e);
                Err(d.error(&err[..]))
            }
        }
    }
}

/// Utility struct of SocketAddrV6 for hole punching
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct SocketAddrV6(pub net::SocketAddrV6);

impl Deref for SocketAddrV6 {
    type Target = net::SocketAddrV6;

    fn deref(&self) -> &net::SocketAddrV6 {
        &self.0
    }
}

impl Encodable for SocketAddrV6 {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        let as_string = format!("{}", self.0);
        s.emit_str(&as_string[..])
    }
}

impl Decodable for SocketAddrV6 {
    fn decode<D: Decoder>(d: &mut D) -> Result<SocketAddrV6, D::Error> {
        let as_string = try!(d.read_str());
        match net::SocketAddrV6::from_str(&as_string[..]) {
            Ok(sa) => Ok(SocketAddrV6(sa)),
            Err(e) => {
                let err = format!("Failed to decode SocketAddrV6: {}", e);
                Err(d.error(&err[..]))
            }
        }
    }
}
