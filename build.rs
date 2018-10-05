#[macro_use]
extern crate clap;

use clap::Shell;
use std::env;

include!("src/bin/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = create_arg_parser();
    app.gen_completions("xxd-rs", Shell::Zsh, outdir);
}
