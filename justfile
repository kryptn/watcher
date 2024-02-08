

fmt-all:
    rg --files -g 'Cargo.toml' | xargs -n1 cargo fmt --manifest-path

build-function fn:
    cargo lambda build --manifest-path functions/{{fn}}/Cargo.toml --arm64 --output-format zip --release
    ls -lah functions/add-endpoint/target/lambda/{{fn}} | grep bootstrap

generate-ci:
    cp cli/github-action.yaml .github/workflows/ci-cli.yaml
    cp watcher/github-action.yaml .github/workflows/ci-watcher.yaml
    cp functions/add-endpoint/github-action.yaml .github/workflows/ci-fn-add-endpoint.yaml