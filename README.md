## Getting Started

First, download htmx and sortable libraries:
```bash
source assets/download.sh 
```

Run the DB:
```bash
docker compose up --build
```
Install Diesel CLI
```bash
cargo install diesel_cli
```

Create .env file (use .env.docker as example) and run migrations:
```bash
diesel migration run
```

Turn on server:
```bash
 cargo watch -x 'run --bin "server"'
```

Also, You might want to populate db with some todos:
```bash
 cargo watch -x 'run --bin "populate_db"'
```

## Web Server
* Rust (Language)
* Actix (Server)
* Tera (HTML Template)
* Diesel (ORM)

## Client
* HTMX
* Sortable by shopify (https://github.com/Shopify/draggable)

