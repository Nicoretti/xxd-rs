extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate xxd;

use clap::{Arg, ArgMatches, App, SubCommand};
use std::process::exit;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;

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
    let input_file = args.value_of("infile").unwrap_or("stdin");
    let seek = usize::from_str_radix(args.value_of("seek").unwrap_or("0"), 10)?;
    let length = args.value_of("length");
    let output_settings = create_output_settings(args)?;
    let reader = create_reader(input_file.to_string())?;
    match length {
        None => dump_iterator(Box::new(reader.bytes().skip(seek)), output_settings),
        Some(n) => {
            dump_iterator(Box::new(reader.bytes().skip(seek).take(usize::from_str_radix(n, 10)?)),
                          output_settings)
        }
    };
    Ok(())
}

fn dump_iterator<I: Iterator<Item = std::result::Result<u8, std::io::Error>>>(it: Box<I>, output_settings: ::xxd::dump::OutputSettings) -> Result<()>{
    let mut data: Vec<u8> = Vec::new();
    let mut address = 0;
    for byte in *it {
        data.push(byte?);
        if data.len() == output_settings.bytes_per_line() {
            dump_line(&data, output_settings.start_address(address));
            address += data.len();
            data.clear();
        }
    }
    if data.len() > 0 {
        dump_line(&data, output_settings.start_address(address));
        address += data.len();
        data.clear();
    }
    Ok(())
}

fn create_output_settings<'a>(args: &ArgMatches<'a>) -> Result<::xxd::dump::OutputSettings> {
    let columns = usize::from_str_radix(args.value_of("columns").unwrap_or("8"), 10)?;
    let format = args.value_of("format").unwrap_or("hex");
    let group_size = usize::from_str_radix(args.value_of("group-size").unwrap_or("1"), 10)?;
    Ok(::xxd::dump::OutputSettings::new()
           .format(::xxd::dump::OutputFormat::from(format.to_string()))
           .group_size(group_size)
           .columns(columns))
}

fn dump_line(data: &[u8], output_settings: ::xxd::dump::OutputSettings) {
    let output_line = ::xxd::dump::OutputLine::new(data).format(output_settings);
    println!("{}", output_line);
}

fn create_reader(path: String) -> Result<Box<std::io::Read>> {
    match path.as_ref() {
        "stdin" => Ok(Box::new(std::io::stdin())),
        _ => {
            let file_reader = std::fs::File::open(path)?;
            Ok(Box::new(file_reader))
        }
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
        .arg(Arg::with_name("seek")
                 .short("s")
                 .long("seek")
                 .required(false)
                 .takes_value(true)
                 .global(true)
                 .help("Offset in the file where to start reading"))
        .arg(Arg::with_name("length")
                 .short("l")
                 .long("length")
                 .required(false)
                 .takes_value(true)
                 .global(true)
                 .help("Amount of bytes which shall be read"))
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
                                 .possible_value("dec")
                                 .help("Specifies the output format for the value (default: hex)"))
                        .arg(Arg::with_name("group-size")
                                 .short("g")
                                 .long("group-size")
                                 .required(false)
                                 .takes_value(true)
                                 .help("Separate  the output of every <bytes> bytes (two hex characters or eight bit-digits each) by a whitespace."))
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
