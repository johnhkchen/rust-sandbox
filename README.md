# rust-sandbox

Just a simple repo demonstrating running a rust package through Docker.

When developing, use the devcontainer for a reliable Rust environment. 

The Dockerfile that comes with the src folder is currently configured to run `main.rs`.

## Running code

After starting the dev container (use Remote Containers plugin in VSCode):

    cargo build
    cargo run

## Testing Code

    cargo test