use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("movie_descriptor.bin"))
        .out_dir("./src/grpc")
        .compile(&["proto/movie.proto"], &["proto"])?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("user_descriptor.bin"))
        .out_dir("./src/grpc")
        .compile(&["proto/user.proto"], &["proto"])?;

    Ok(())
}
