fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=./src");

    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .type_attribute(
            "DetailedError",
            "#[derive(serde::Deserialize, serde::Serialize)]",
        )
        .type_attribute(
            "ErrorCode",
            "#[derive(strum::Display, serde::Deserialize, serde::Serialize)]",
        )
        .compile(
            &[
                "authcore.proto",
                "auth/basic.proto",
                "session.proto",
                "error.proto",
            ],
            &["../../protos/authcore"],
        )?;

    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile(&["email.proto"], &["../../protos/messaging"])?;

    Ok(())
}
