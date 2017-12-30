#[macro_use]
extern crate common_failures;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate parity_wasm;
extern crate rustc_demangle;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use common_failures::prelude::*;
use parity_wasm::elements::{Deserialize, Section};
use std::io::Cursor;
use std::path::PathBuf;
use structopt::StructOpt;

mod index_map;
mod name_section;

use name_section::*;

/// Our command-line arguments.
#[derive(Debug, StructOpt)]
#[structopt(name = "wasm-bloat", about = "Analyze a wasm file, and determine what's using all the space.")]
struct Opt {
    /// The *.wasm file to analyze.
    #[structopt(parse(from_os_str))]
    wasm_file: PathBuf,
}

quick_main!(run);

fn run() -> Result<()> {
    env_logger::init();

    // Parse our command-line arguments.
    let opt = Opt::from_args();
    debug!("Arguments: {:?}", opt);

    // Load our *.wasm file.
    let module = parity_wasm::deserialize_file(&opt.wasm_file)
        .map_err(|err| {
            // TODO: Implement `std::error::Error` for `parity_wasm::Error`
            // and submit upstream.
            format_err!("couldn not parse *.wasm: {:?}", err)
        })
        .io_read_context(&opt.wasm_file)?;

    // Find the custom section containing function names (which you'll probably
    // need to go out of your way to generate).
    let mut function_names_opt = None;
    for s in module.sections() {
        match *s {
            Section::Custom(ref cs) if cs.name() == "name" => {
                let mut cursor = Cursor::new(cs.payload());
                let section = NameSection::deserialize(&mut cursor).map_err(|err| {
                    format_err!("couldn not parse name section: {:?}", err)
                })?;
                if let NameSection::Function(function_names) = section {
                    function_names_opt = Some(function_names);
                }
            }
            _ => {},
        }
    }
    let function_names = function_names_opt.ok_or_else(|| {
        format_err!("no names found, try compiling with `-C debuginfo=1`")
    })?;

    for (_idx, name) in function_names.names() {
        println!("{}", name);
    }

    Ok(())
}
