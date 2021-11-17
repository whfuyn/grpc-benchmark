// We are using `real_async_trait` which requires us to
// manually fix the generated file.

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("cargo:rerun-if-changed=../proto");
//     tonic_build::configure()
//         .build_client(true)
//         .build_server(true)
//         .format(true)
//         .out_dir("src")
//         .compile(
//             &["example.proto"],
//             &["../proto"],
//         )?;
//     Ok(())
// }
