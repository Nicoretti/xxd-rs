#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

use error_chain::*;
use std::fmt;
use std::result;
use std::fmt::Error;

pub mod dump;
pub mod generate;
pub mod convert;

pub mod errors {
    error_chain!{
        foreign_links {
            FormatError(::std::fmt::Error);
            Io(::std::io::Error);
        }
    }
}

use errors::*;
