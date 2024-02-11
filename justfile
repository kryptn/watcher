fmt-all:
    rg --files -g 'Cargo.toml' | xargs -n1 cargo fmt --manifest-path

build-function fn:
    cargo lambda build --manifest-path functions/{{fn}}/Cargo.toml --arm64 --output-format zip --release
    ls -lah functions/add-endpoint/target/lambda/{{fn}} | grep bootstrap

generate-ci:
    #!/bin/bash
    for action in `rg --files -g 'github-action.yaml'`; do
        cp $action .github/workflows/ci-$(basename $(dirname $action)).yaml
    done

ws_file_default:="watcher.code-workspace"

update-workspace ws_file=ws_file_default:
    rg --files -g 'Cargo.toml' | jq -Rn '.settings."rust-analyzer.linkedProjects" = [inputs | ("./" + .)]' > projects.json
    jq -s '.[0] * .[1]' {{ws_file}} projects.json > tmp.{{ws_file}}
    rm projects.json
    mv tmp.{{ws_file}} {{ws_file}}
