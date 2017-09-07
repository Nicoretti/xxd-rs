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

pub fn create_reader(path: String) -> Result<Box<std::io::Read>> {
    match path.as_ref() {
        "stdin" => Ok(Box::new(std::io::stdin())),
        _ => {
            let file_reader = std::fs::File::open(path)?;
            Ok(Box::new(file_reader))
        }
    }
}

pub fn create_writer(path: String) -> Result<Box<std::io::Write>> {
    match path.as_ref() {
        "stdout" => Ok(Box::new(std::io::stdout())),
        _ => {
            let mut file_writer = std::fs::File::create(path)?;
            Ok(Box::new(file_writer))
        }
    }
}
