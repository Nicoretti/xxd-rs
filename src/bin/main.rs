extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate xxd;

use clap::{Arg, ArgMatches, App, SubCommand};
use error_chain::*;
use std::io::prelude::*;
use std::process::exit;
use std::fs::File;
use std::fmt::Display;
use std::isize::*;
use std::usize::*;
use std::num::ParseIntError;

mod errors {
    error_chain! {
        foreign_links {
            ParseError(::std::num::ParseIntError);
            Xxd(::xxd::errors::Error);
        }
    }
}

use errors::*;

fn main() {
    let matches = create_arg_parser().get_matches();
    match run(matches) {
        Ok(_) => exit(0),
        Err(e) => {
            report_error(e);
            exit(1)
        }
    }
}

fn run<'a>(args: ArgMatches<'a>) -> Result<()> {
    match args.subcommand_name() {
        Some("dump") => dump(args.subcommand_matches("dump")),
        Some("convert") => convert(args.subcommand_matches("convert")),
        Some("generate") => generate(args.subcommand_matches("generate")),
        _ => bail!(args.usage()),
    }
}

fn dump<'a>(args: Option<&ArgMatches<'a>>) -> Result<()> {
    let args = args.ok_or("No arguments available")?;
    // TODO NiCo: consider using stdin as default instead of reporting an error.
    let input_file = args.value_of("infile").ok_or("No input file specified")?;
    let columns = usize::from_str_radix(args.value_of("columns").unwrap_or("8"), 10)?;
    let format = args.value_of("columns").unwrap_or("hex");
    let seek = usize::from_str_radix(args.value_of("seek").unwrap_or("0"), 10)?;
    let length = isize::from_str_radix(args.value_of("length").unwrap_or("-1"), 10)?;
    // TODO NiCo: add group size as paramter
    let group_size = 1;
    println!("in-file: {}, columns: {}, format: {}, seek: {}, length: {}, group size: {}",
             input_file,
             columns,
             format,
             seek,
             length,
             group_size);
    // Read file chunk by chunk (chunk size = columns * group_size)
    // create output line for each chunk
    Ok(())
}

fn convert<'a>(args: Option<&ArgMatches<'a>>) -> Result<()> {
    command_not_supported()
}

fn generate<'a>(args: Option<&ArgMatches<'a>>) -> Result<()> {
    command_not_supported()
}

fn command_not_supported() -> Result<()> {
    bail!("Command not supported yet!")
}

fn report_error<T: Display>(error: T) {
    println!("xxd-rs: {}", error)
}

fn create_arg_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("A rust based clone of the all time classic xxd tool")
        .version("0.1.0")
        .author("Nicola Coretti <nicola.coretti@gmail.com>")
        .about("make a hexdump or the reverse")
        .arg(Arg::with_name("infile")
                 .required(false)
                 .global(true)
                 .index(1)
                 .help("Input file which shall be used (default: stdin)"))
        .arg(Arg::with_name("outfile")
                 .required(false)
                 .global(true)
                 .index(2)
                 .help("File to which the output will be written (default: stdout)"))
        .arg(Arg::with_name("length")
                 .short("l")
                 .long("length")
                 .required(false)
                 .takes_value(true)
                 .global(true)
                 .help("Amount of bytes which shall be read"))
        .arg(Arg::with_name("seek")
                 .short("s")
                 .long("seek")
                 .required(false)
                 .takes_value(true)
                 .global(true)
                 .help("Offset in the file where to start reading"))
        .subcommand(SubCommand::with_name("dump")
                        .about("Dumps an input file in the appropriate output format")
                        .arg(Arg::with_name("format")
                                 .short("f")
                                 .long("format")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("hex")
                                 .possible_value("bin")
                                 .possible_value("oct")
                                 .help("Specifies the output format for the value (default: hex)"))
                        .arg(Arg::with_name("columns")
                                 .short("c")
                                 .long("columns")
                                 .required(false)
                                 .takes_value(true)
                                 .help("Specifies the amount of output columns")))
        .subcommand(SubCommand::with_name("convert")
                        .about("Converts input data to a file (e.g. hexstream -> binary file")
                        .arg(Arg::with_name("format")
                                 .short("f")
                                 .long("format")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("binary")
                                 .possible_value("c-array")
                                 .possible_value("oct")
                                 .help("Specifies the output format (default: hex)")))
        .subcommand(SubCommand::with_name("generate")
                        .about("Generates a source file containing the specified file as array")
                        .arg(Arg::with_name("template")
                                 .short("t")
                                 .long("template")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("c")
                                 .possible_value("cpp")
                                 .possible_value("rs")
                                 .help("Specifies a template file which shall be used for \
                                        generation"))
                        .arg(Arg::with_name("format")
                                 .short("f")
                                 .long("format")
                                 .required(false)
                                 .takes_value(true)
                                 .possible_value("hex")
                                 .possible_value("oct")
                                 .possible_value("dec")
                                 .possible_value("bin")
                                 .help("Specifies the output format (default: hex)"))
                        .arg(Arg::with_name("Separator")
                                 .long("separator")
                                 .required(false)
                                 .takes_value(true)
                                 .help("Specifies the the separator between single values \
                                        (default: ',')")))
}
