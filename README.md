
# Rust GraphQL API with ScyllaDB

This project is a GraphQL API server built in **Rust** using `async-graphql` and `axum`, with persistent storage powered by **ScyllaDB**.

It allows you to create and retrieve user records via GraphQL endpoints.

---

## 🚀 Features

- GraphQL server built with `async-graphql` and `axum`
- Database connection to ScyllaDB via `scylla` driver
- Uses environment variables from `.env` file
- Logging via `env_logger`
- Async and production-ready

---

## 🛠️ Technologies Used

- Rust
- async-graphql
- axum
- ScyllaDB (via `scylla` crate)
- dotenvy
- env_logger

---

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70+ recommended)
- [Docker](https://www.docker.com/) (for running ScyllaDB locally)

---

## 📦 Environment Variables

Create a `.env` file in the project root:

```env
DATABASE_URL=127.0.0.1:9042
KEYSPACE=my_keyspace
```

---

## 🧪 Run ScyllaDB Locally

```bash
docker run --name scylla -d -p 9042:9042 scylladb/scylla
```

Then create the keyspace and table:

```sql
CREATE KEYSPACE IF NOT EXISTS my_keyspace
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};

USE my_keyspace;

CREATE TABLE IF NOT EXISTS users (
  id int PRIMARY KEY,
  name text
);
```

You can apply this using `cqlsh`.

---

## ▶️ How to Run

```bash
cargo build
cargo run
```

The server will be available at: [http://localhost:8080/graphql](http://localhost:8080/graphql)

---

## 📥 Insert a User (Mutation)

**PowerShell Example:**

```powershell
Invoke-WebRequest -Uri "http://localhost:8080/graphql" `
  -Method POST `
  -Headers @{ "Content-Type" = "application/json" } `
  -Body '{"query":"mutation { create_user(input: { id: 1, name: "Alice" }) { id name } }"}'
```

---

## 📤 Retrieve a User (Query)

**PowerShell Example:**

```powershell
Invoke-WebRequest -Uri "http://localhost:8080/graphql" `
  -Method POST `
  -Headers @{ "Content-Type" = "application/json" } `
  -Body '{"query":"query { user(id: 1) { id name } }"}'
```

---

## ✅ Output Example

```json
{
  "data": {
    "user": {
      "id": 1,
      "name": "Alice"
    }
  }
}
```

---

## 📄 License

MIT License.
