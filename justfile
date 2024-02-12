
artifact_bucket := env_var('ARTIFACT_BUCKET')

fmt-all:
    rg --files -g 'Cargo.toml' | xargs -n1 cargo fmt --manifest-path



build-function fn:
    #!/bin/bash
    cargo lambda build --manifest-path functions/{{fn}}/Cargo.toml --arm64 --output-format zip --release
    ls -lah functions/{{fn}}/target/lambda/{{fn}} | grep bootstrap

prove:
    #!/bin/bash
    echo "{{artifact_bucket}}"

push-artifact fn version:
    #!/bin/bash
    aws s3 cp functions/{{fn}}/target/lambda/{{fn}}/bootstrap.zip s3://{{artifact_bucket}}/lambda/{{fn}}-{{version}}/bootstrap.zip

push-artifact-commit fn:
    #!/bin/bash
    commit=$(git rev-parse HEAD)
    just push-artifact {{fn}} $commit

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
