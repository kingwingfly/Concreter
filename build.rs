fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // .out_dir("src/pb")  // use `tonic::include_proto!` instead
        .build_server(false)
        .compile(&["proto/sym.proto"], &["./proto"])
        .unwrap();
    Ok(())
}
