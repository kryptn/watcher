



build-function fn:
    cargo lambda build --manifest-path functions/{{fn}}/Cargo.toml --arm64 --output-format zip --release
    ls -lah functions/add-endpoint/target/lambda/{{fn}} | grep bootstrap