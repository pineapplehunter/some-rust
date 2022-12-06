use cc::Build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=boot.s");
    println!("cargo:rerun-if-changed=linker.ld");
    // assemble the `boot.s` file
    Build::new()
        .compiler("riscv64-unknown-elf-gcc")
        .file("boot.s")
        .flag("-mabi=lp64")
        .compile("asm");

    Ok(())
}
