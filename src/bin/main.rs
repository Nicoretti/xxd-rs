#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate clap;
extern crate failure;
extern crate xxd;

use cli::create_arg_parser;
use failure::*;
use xxd::dump::{dump_iterator, Config, Format};
use xxd::generate::{Render, Template};

use clap::ArgMatches;

use std::fmt::Display;
use std::io::{Read, Write};
use std::process::exit;

mod cli;

fn main() {
    setup_panic!();
    let matches = create_arg_parser().get_matches();
    match run(&matches) {
        Ok(_) => exit(0),
        Err(e) => {
            report_error(&e);
            exit(1)
        }
    }
}

fn run(args: &ArgMatches) -> Result<(), failure::Error> {
    match args.subcommand_name() {
        Some("dump") => dump(args.subcommand_matches("dump")),
        Some("generate") => generate(args.subcommand_matches("generate")),
        _ => bail!(format_err!("{}", args.usage())),
    }
}

pub fn create_reader(path: String) -> Result<Box<std::io::Read>, failure::Error> {
    match path.as_ref() {
        "stdin" => Ok(Box::new(std::io::stdin())),
        _ => {
            let file_reader = std::fs::File::open(path)?;
            Ok(Box::new(file_reader))
        }
    }
}

pub fn create_writer(path: String) -> Result<Box<std::io::Write>, failure::Error> {
    match path.as_ref() {
        "stdout" => Ok(Box::new(std::io::stdout())),
        _ => {
            let mut file_writer = std::fs::File::create(path)?;
            Ok(Box::new(file_writer))
        }
    }
}

fn dump<'a>(args: Option<&ArgMatches<'a>>) -> Result<(), failure::Error> {
    let args = args.ok_or_else(|| format_err!("No arguments available"))?;
    let output_file = args.value_of("outfile").unwrap_or("stdout");
    let input_file = args.value_of("file").unwrap_or("stdin");
    let seek = usize::from_str_radix(args.value_of("seek").unwrap_or("0"), 10)?;
    let length = args.value_of("length");
    let settings = create_dump_settings(args)?;
    let reader = create_reader(input_file.to_string())?;
    let mut writer = create_writer(output_file.to_string())?;
    match length {
        None => dump_iterator(
            reader.bytes().skip(seek).flat_map(|result| result),
            &mut *writer,
            settings,
        ),
        Some(n) => dump_iterator(
            reader
                .bytes()
                .skip(seek)
                .take(usize::from_str_radix(n, 10)?)
                .flat_map(|result| result),
            &mut *writer,
            settings,
        ),
    }
}

fn create_dump_settings<'a>(args: &ArgMatches<'a>) -> Result<Config, failure::Error> {
    let columns = usize::from_str_radix(args.value_of("columns").unwrap_or("8"), 10)?;
    let format = args.value_of("format").unwrap_or("hex");
    let address = usize::from_str_radix(args.value_of("seek").unwrap_or("0"), 10)?;
    let group_size = usize::from_str_radix(args.value_of("group-size").unwrap_or("2"), 10)?;
    let settings = Config::new()
        .format(Format::from(format.to_string()))
        .group_size(group_size)
        .columns(columns)
        .set_address(address);
    if args.is_present("plain_hexdump") {
        Ok(settings
            .separator(false)
            .show_address(false)
            .show_interpretation(false))
    } else {
        Ok(settings)
    }
}

fn generate<'a>(args: Option<&ArgMatches<'a>>) -> Result<(), failure::Error> {
    let args = args.ok_or_else(|| format_err!("No arguments available"))?;
    let output_file = args.value_of("outfile").unwrap_or("stdout");
    let input_file = args.value_of("file").unwrap_or("stdin");
    let seek = usize::from_str_radix(args.value_of("seek").unwrap_or("0"), 10)?;
    let length = args.value_of("length");
    let reader = create_reader(input_file.to_string())?;
    let mut writer = create_writer(output_file.to_string())?;
    let lang = xxd::generate::Language::from(args.value_of("template").unwrap_or("c"));
    let template = Template::new(lang);
    let data: Vec<u8> = match length {
        None => reader
            .bytes()
            .skip(seek)
            .flat_map(|result| result)
            .collect(),
        Some(n) => reader
            .bytes()
            .skip(seek)
            .take(usize::from_str_radix(n, 10)?)
            .flat_map(|result| result)
            .collect(),
    };
    writer.write_fmt(format_args!("{}\n", template.render(&data)))?;
    Ok(())
}

fn report_error<T: Display>(error: &T) {
    eprintln!("xxd-rs: {}", error);
}
