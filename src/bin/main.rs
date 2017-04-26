extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate xxd;

use cli::create_arg_parser;
use xxd::{create_reader, create_writer};
use xxd::dump::{dump_iterator, OutputSettings, OutputFormat};

use clap::ArgMatches;

use std::process::exit;
use std::fmt::Display;
use std::io::{Read, Write, stderr};

mod cli;
mod errors {
    error_chain! {
        foreign_links {
            ParseError(::std::num::ParseIntError);
            Xxd(::xxd::errors::Error);
            Io(::std::io::Error);
        }
    }
}

use errors::*;

fn main() {
    let matches = create_arg_parser().get_matches();
    match run(&matches) {
        Ok(_) => exit(0),
        Err(e) => {
            report_error(&e);
            exit(1)
        }
    }
}

fn run(args: &ArgMatches) -> Result<()> {
    match args.subcommand_name() {
        Some("dump") => dump(args.subcommand_matches("dump")),
        Some("convert") => convert(args.subcommand_matches("convert")),
        Some("generate") => generate(args.subcommand_matches("generate")),
        _ => bail!(args.usage()),
    }
}

fn dump<'a>(args: Option<&ArgMatches<'a>>) -> Result<()> {
    let args = args.ok_or("No arguments available")?;
    let output_file = args.value_of("outfile").unwrap_or("stdout");
    let input_file = args.value_of("infile").unwrap_or("stdin");
    let seek = usize::from_str_radix(args.value_of("seek").unwrap_or("0"), 10)?;
    let length = args.value_of("length");
    let settings = create_dump_settings(args)?;
    let reader = create_reader(input_file.to_string())?;
    let mut writer = create_writer(output_file.to_string())?;
    match length {
        None => dump_iterator(Box::new(reader.bytes().skip(seek)), &mut *writer, settings),
        Some(n) => {
            dump_iterator(Box::new(reader.bytes().skip(seek).take(usize::from_str_radix(n, 10)?)),
                          &mut *writer,
                          settings)
        }
    };
    Ok(())
}

fn create_dump_settings<'a>(args: &ArgMatches<'a>) -> Result<OutputSettings> {
    let columns = usize::from_str_radix(args.value_of("columns").unwrap_or("8"), 10)?;
    let format = args.value_of("format").unwrap_or("hex");
    let group_size = usize::from_str_radix(args.value_of("group-size").unwrap_or("2"), 10)?;
    let mut settings = OutputSettings::new()
        .format(OutputFormat::from(format.to_string()))
        .group_size(group_size)
        .columns(columns);
    if args.is_present("plain_hexdump") {
        Ok(settings.separator(false).show_address(false).show_interpretation(false))
    } else {
        Ok(settings)
    }
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

fn report_error<T: Display>(error: &T) {
    std::io::stderr().write_fmt(format_args!("xxd-rs: {}\n", error));
}
