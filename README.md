[![Build Status](https://travis-ci.org/Nicoretti/xxd-rs.svg?branch=master)](https://travis-ci.org/Nicoretti/xxd-rs)

# xxd-rs
A platform independent reimplementation in rust of Juergen Weigert's hexdump utility, xxd.

```
USAGE:
    xxd-rs [OPTIONS] [ARGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>    Amount of bytes which shall be read
    -s, --seek <seek>        Offset in the file where to start reading

ARGS:
    <infile>     Input file which shall be used (default: stdin)
    <outfile>    File to which the output will be written (default: stdout)

SUBCOMMANDS:
    convert     Converts input data to a file (e.g. hexstream -> binary file
    dump        Dumps an input file in the appropriate output format
    generate    Generates a source file containing the specified file as array
    help        Prints this message or the help of the given subcommand(s)
```
