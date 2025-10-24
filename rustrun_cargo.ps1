$pathToManifest = $Args[0] + "\Cargo.toml"
cargo run --manifest-path $pathToManifest -- $Args[1]