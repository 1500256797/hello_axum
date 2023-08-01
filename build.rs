use std::path::PathBuf;
fn main() {
    let gencode_out_dir = PathBuf::from("src/grpc");

    tonic_build::configure()
        .out_dir(gencode_out_dir)
        .compile(&["proto/hello_axum.proto"], &["proto"])
        .unwrap();
}
