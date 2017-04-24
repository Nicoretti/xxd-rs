#[macro_use]
extern crate error_chain;

use error_chain::*;
use std::fmt;
use std::result;
use std::fmt::Error;

pub mod dump;
pub mod generate;

pub mod errors {
    error_chain!{
        foreign_links {
            FormatError(::std::fmt::Error);
        }
    }
}
