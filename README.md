# interface lab

Three API interfaces built with [actix-web] and [diesel]. 

[actix-web]: https://github.com/actix/actix-web
[diesel]: https://github.com/diesel-rs/diesel

## Deploy
Install Rust:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"

Setup database:

    sqlite3 users.db < ./schema.sql

Then run:

    cargo run --release
