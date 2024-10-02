use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["../../wrapper.proto"], &["src/"])?;
    Ok(())
}