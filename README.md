# interface lab

## Deploy
Install Rust:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"

Setup database:

    sqlite3 users.db < ./schema.sql

Then run:

    cargo run --release
