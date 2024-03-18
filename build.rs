const LINKER_SCRIPT: &'static str = "linker.ld";
fn main() {
    println!("cargo:rerun-if-changed={LINKER_SCRIPT}");
    println!("cargo:rustc-link-arg=-T{LINKER_SCRIPT}");
    println!("cargo:rustc-link-arg=-zmax-page-size=1024");
}
