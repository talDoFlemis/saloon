use std::io::Result;

fn main() -> Result<()> {
    let protos = ["protocol/raft.proto"];
    let includes = ["src/", "protocol/"];
    prost_build::compile_protos(&protos, &includes)?;
    Ok(())
}
