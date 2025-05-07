fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // adjust to your .proto location:
        .compile(
            &["proto/services.proto"],
            &["proto"],
        )?;
    Ok(())
}