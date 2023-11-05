fn main() -> Result<(), std::io::Error> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_well_known_types(true)
        .compile(
            &["api.proto"], // Files in the path
            &["./protos"],  // The path to search
        )
        .unwrap();

    Ok(())
}
