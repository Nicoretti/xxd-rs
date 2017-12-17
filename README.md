[![Build Status](https://travis-ci.org/Nicoretti/xxd-rs.svg?branch=master)](https://travis-ci.org/Nicoretti/xxd-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/nki2w285pjq73jhk/branch/master?svg=true)](https://ci.appveyor.com/project/Nicoretti/xxd-rs/branch/master)

# xxd-rs
A rust based reimplementation of [Juergen Weigert's](jnweiger@informatik.uni-erlangen.de) hexdump utility, xxd.

## Mission statement
This project was created to learn rust, therefore there is no perf
If you wanna use the proect or contribute feel free, but please take note
of the goal(s) and non goals so you won't waste ur time or get frustrated.

### Goals
1. Learn rust
2. Provide a rust based replacement for xxd
3. Strive towards a clean rust code base
    - rustfmt
    - Add/refactor towards common rust idioms
4. Useability
    - Provide user friendly defaults
    - Provide clear and well documented command line tools
5. Continously improve the code base


### Non Goals
What this Project isn't about

1. Implement the fastest dump utility out there
    - If you are looking for a performant implementation of xxd
      checkout [go-xxd](https://github.com/felixge/go-xxd)

## Usage
```
USAGE:
    xxd-rs [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input-file <infile>      Input file which shall be used (default: stdin)
    -l, --length <length>          Amount of bytes which shall be read
    -o, --output-file <outfile>    File to which the output will be written (default: stdout)
    -s, --seek <seek>              Offset in the file where to start reading

SUBCOMMANDS:
    dump        Dumps an input file in the appropriate output format
    generate    Generates a source file containing the specified file as array
    help        Prints this message or the help of the given subcommand(s)
```

## Examples
1. Dump file
```
user@host:~$ xxd-rs dump -i file.txt
```

2. Dump file with 16 bit word size
```
user@host:~$ xxd-rs generate -g 2 -i file.txt
```

3. Dump 1024 bytes of file file starting at offset 128
```
user@host:~$ xxd-rs dump -s 128 -l 1024 -i file.txt
```

4. Generate cpp header file containing file in an cpp array
```
user@host:~$ xxd-rs generate -t cpp -i file.txt
```

## Migration/Compatibility
Be aware that the output formats (especially the default) of hexdump, xxd, od, and xxd-rs differ.

1. Dump file
    1. xxd
    ```
    user@host:~$ xxd file
    ```
    2. od
    ```
    user@host:~$ od file
    ```
    3. hexdump
    ```
    user@host:~$ xxd-rs dump -i file
    ```
    4. xxd-rs
    ```
    user@host:~$ xxd-rs dump -i file
    ```

## Screenshots
### hex dump
![hex_dump_upper](resources/screen_shot_hex_upper.png)
![hex_dump_lower](resources/screen_shot_hex_lower.png)
### binary dump
![binary_dump](resources/screen_shot_bin.png)

### octal dump
![octal_dump](resources/screen_shot_oct.png)

### decimal dump
![octal_dump](resources/screen_shot_dec.png)

### plain dump
![plain_dump](resources/screen_shot_plain.png)

