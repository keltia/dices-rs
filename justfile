# Fetch from remote origin using jj
pull:
    jj git fetch --remote origin

# Push to github
push-github:
    jj git push --tracked --remote origin

# Push to both github and gitlab
push: push-github

# Move changes to develop
move:
    jj b move -t '@-' develop

build:
	cargo build

install: build
	cargo install --path .
