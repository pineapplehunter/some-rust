use cc::Build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // assemble the `boot.s` file
    Build::new()
        .compiler("riscv64-unknown-elf-gcc")
        .file("boot.s")
        .flag("-mabi=lp64")
        .compile("asm");

    Ok(())
}
