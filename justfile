[windows]
set shell := ["C:\\Program Files\\Git\\bin\\sh.exe", "-c"]

# Fetch from remote origin using jj
pull:
    jj git fetch --remote origin

push: push-github push-codeberg

# Push to github
push-github:
    jj git push --tracked --remote origin

# Push to codeberg
push-codeberg:
    jj git push --tracked --remote codeberg

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
