fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = cxx_build::bridge("src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
