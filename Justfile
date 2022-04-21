login:
    cargo login

publish:
    #!/bin/bash
    # git clean -dfxq
    version="$(cat Cargo.toml | grep '^version = ' | cut -d ' ' -f3 | sed 's/\"//g')"
    git tag -a $version -m "Version ${version}"
    git push origin ${version}
    cargo publish

doc:
    cargo doc --package gdunsafe --no-deps --open
