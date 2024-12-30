# Todo API in Rust

A simple Todo API implemented in Rust using Actix and Diesel.

## Tech Stack

- [Rust](https://www.rust-lang.org/) - Programming Language
- [Actix Web](https://actix.rs/) - Web Framework
- [Diesel](https://diesel.rs/) - ORM and Query Builder
- [PostgreSQL](https://www.postgresql.org/) - Database

## Project Structure 
```
.
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── README.md
├── compose.yaml
├── diesel.toml
├── migrations
│   ├── .keep
│   ├── 00000000000000_diesel_initial_setup
│   │   ├── down.sql
│   │   └── up.sql
│   └── 2024-12-28-122024_create_todo_tasks
│       ├── down.sql
│       └── up.sql
├── src
│   ├── handlers.rs # API handler implementations
│   ├── lib.rs # Library root module
│   ├── main.rs # Application entry point
│   ├── models.rs # Data model definitions
│   └── schema.rs # Generated schema definitions by Diesel
└── tests
    └── handlers_test.rs # Integration tests
```

## Development Setup

### Prerequisites
- Rust (via rustup)
- Clippy (Rust linter)

### Setup Steps
1. Clone the repository
```bash
git clone https://github.com/horizon67/todo-rust.git
```

2. Create Docker network
```bash
docker network create app_network
```

3. Start database with Docker
```bash
docker-compose up -d
```

4. Run migrations
```
docker compose run --rm app diesel migration run
```

5. Install Clippy
```bash
docker compose run --rm app rustup component add clippy
```

6. Install dependencies
```bash
docker compose run --rm app cargo build
```

## Testing
```bash
docker compose run --rm app cargo test
```

### Linting
```bash
docker compose run --rm app cargo clippy
```

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/tasks` | Get all tasks |
| POST | `/tasks` | Create a new task |
| PUT | `/tasks/{id}` | Update a specific task |
| DELETE | `/tasks/{id}` | Delete a specific task |


## Request Examples
### Create a Task
```
curl -X POST http://localhost:8080/tasks \
-H "Content-Type: application/json" \
-d '{"content": "New task", "state": 0}'
```

### Update a Task
```
curl -X PUT http://localhost:8080/tasks/1 \
-H "Content-Type: application/json" \
-d '{"content": "Updated task", "state": 1}'
```

### Get All Tasks
```bash
curl http://localhost:8080/tasks
```

### Delete a Task
```bash
curl -X DELETE http://localhost:8080/tasks/1
```
