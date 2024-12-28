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

## API Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/tasks` | Get all tasks |
| POST | `/tasks` | Create a new task |
| PUT | `/tasks/{id}` | Update a specific task |
| DELETE | `/tasks/{id}` | Delete a specific task |

## Start database with Docker
```
docker-compose up -d
```

## Run migrations
```
diesel migration run
```

## Testing
```
Run all tests
```

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
