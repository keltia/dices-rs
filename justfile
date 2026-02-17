# Fetch from remote origin using jj
pull:
    jj git fetch --remote origin

# Push to github
push:
    jj git push --tracked --remote origin

# Move changes to develop
move:
    jj b move -t '@-' develop

build:
    cargo build

test:
    cargo nextest run

build-release:
    cargo build --release

install: build-release
    cargo install --path .
