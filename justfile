set dotenv-load
image_name := "ghcr.io/babsyit/onboarder"


# Perform all verifications (compile, test, lint, etc.)
@verify: test lint
    echo ------------ verify done! ------------  

run *args:
    cargo run -q -- {{args}}

install:
    cargo install --path .
    
rr:
    onboarder
    
kill:
    kill $(lsof -t -i:8080)
    
# Watch the source files and run `just verify` when source changes
watch:
	cargo watch -- just run

# Run tests    
test:
    cargo test

# Run the static code analysis
lint:
    cargo fmt --all -- --check
    cargo clippy

fmt:
    cargo fmt

build:
    cargo build

up *args:
    docker compose down 
    docker-compose up {{args}}
    

push:
    just _d_push $(just get_version)
    
_d_push version:
    docker build -t {{image_name}}:{{version}} . 
    docker push {{image_name}}:{{version}}

get_version:
    cargo metadata --no-deps --format-version 1 | jq -r '.packages.[0].version'