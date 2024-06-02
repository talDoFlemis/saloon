use std::io::Result;

fn main() -> Result<()> {
    let protos = ["protocol/raft.proto"];
    let includes = ["protocol", "src/"];
    tonic_build::configure()
        .build_server(true)
        .compile(&protos, &includes)?;
    Ok(())
}
