#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

//! xauth-rs - Helper library for working with X11 security protocols
//!
//! This crates intends to be an option to libXau in pure Rust.
//!
//! # Usage
//! *WARNING* this is a proof of concept library and it isn't heavily tested
//! so please be carefully, if you don't understand something please open an
//! isssue on the **GitHub** repo.
//!
//! As this project isn't very stable you need to add as a git repo on cargo:
//! ```toml
//! [dependencies]
//! xauth = { git = "https://www.github.com/jeandudey/xauth-rs/" }
//! ```
//!
//! # Notes
//! This crate doesn't implement the lock, unlock and write functionality, if you want it
//! just ask and i will see what i can do.

extern crate byteorder;

use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use byteorder::{ReadBytesExt, BigEndian};

/// This represents the authentication information.
#[derive(Debug)]
pub struct Xauth {
    /// The authentication family
    pub family: Family,

    /// Don't known usage
    pub address: Vec<u8>,

    /// Don't known usage
    pub number: Vec<u8>,

    /// The authentication protocol name.
    pub name: Vec<u8>,

    /// The authentication data.
    pub data: Vec<u8>,
}

/// The authentication family.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Family {
    /// Not part of X standard.
    Local,

    /// Wild
    Wild,

    /// Not part of X standard.
    Netname,

    /// Kerberos 5 principal name.
    Krb5Principal,

    /// For local, non net, authentication.
    LocalHost,
}

const FAMILY_LOCAL: u16 = 256;
const FAMILY_WILD: u16 = 65535;
const FAMILY_NETNAME: u16 = 254;
const FAMILY_KRB5PRINCIPAL: u16 = 253;
const FAMILY_LOCALHOST: u16 = 252;

impl Family {
    fn from_raw(family: u16) -> Option<Family> {
        match family {
            FAMILY_LOCAL => Some(Family::Local),
            FAMILY_WILD => Some(Family::Wild),
            FAMILY_NETNAME => Some(Family::Netname),
            FAMILY_KRB5PRINCIPAL => Some(Family::Krb5Principal),
            FAMILY_LOCALHOST => Some(Family::LocalHost),
            _ => None,
        }
    }
}

impl Xauth {
    /// Returns the XAuthority path.
    pub fn get_path() -> Result<PathBuf, env::VarError> {
        match env::var("XAUTHORITY") {
            Ok(name) => return Ok(PathBuf::from(name)),
            Err(_) => (),
        }

        let home = try!(env::var("HOME"));
        let mut path = Path::new(&home).to_path_buf();
        path.push(".Xauthority");

        Ok(path)
    }

    /// Reads the specified Xauthority file.
    pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Xauth> {
        let mut file = try!(File::open(path));
        let family_raw = try!(file.read_u16::<BigEndian>());
        let address = try!(read_counted_string(&mut file));
        let number = try!(read_counted_string(&mut file));
        let name = try!(read_counted_string(&mut file));
        let data = try!(read_counted_string(&mut file));

        let family = if let Some(f) = Family::from_raw(family_raw) {
            f
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid family"));
        };

        Ok(Xauth {
            family: family,
            address: address,
            number: number,
            name: name,
            data: data,
        })
    }
}

fn read_counted_string<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
    let len = try!(reader.read_u16::<BigEndian>());

    let mut string: Vec<u8> = Vec::new();
    try!(reader.take(len as u64).read_to_end(&mut string));
    Ok(string)
}
