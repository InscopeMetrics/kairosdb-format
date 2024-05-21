use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &["../src/main/proto/format.v2.proto"],
        &["../src/main/proto"],
    )?;
    Ok(())
}
