extern crate cargo_binutils as cbu;

use std::process;

fn main() {
    match cbu::forward("llvm-objcopy") {
        Err(e) => eprintln!("error: {}", e),
        Ok(ec) => process::exit(ec),
    }
}
