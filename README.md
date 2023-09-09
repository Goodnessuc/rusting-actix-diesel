# 🛠️ actix-diesel-rocket - Rust with Actix-web and Diesel

A simple API built using Rust's Actix-web framework and Diesel ORM for interacting with `Human` data.

## 📑 Overview

The project demonstrates CRUD (Create, Read, Update, Delete) operations for a `Human` entity. The operations include:

- Creating a new human.
- Reading all humans from the database.
- Updating an existing human's details.
- Deleting a human.

## 🌟 Features

- Built with the Actix-web framework for asynchronous web handling.
- Uses Diesel ORM for database operations.
- SQLite as the database.
- Clean and clear route definitions.
- Structured error handling.

## 🧰 Prerequisites

- Rust and Cargo installed.
- SQLite3

## 🚀 Setup & Run

1. Clone the repository:

```
git clone <repository-url>
```

2. Navigate to the project directory:

```
cd <project-name>
```

3. Run migrations to setup the database:

```
diesel migration run
```

4. Start the server:

```
cargo run
```

The server will start at `127.0.0.1:8080`.

## 📜 API Endpoints

### 🧑 Create a New Human

- **Method**: `POST`
- **Endpoint**: `/human`
- **Body**:

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "age": 30
}
```

### 📖 Read All Humans

- **Method**: `GET`
- **Endpoint**: `/humans`

### ✏️ Update a Human

- **Method**: `PUT`
- **Endpoint**: `/human/{id}`
- **Body**:

```json
{
  "first_name": "Jane",
  "last_name": "Doe",
  "age": 32
}
```

### ❌ Delete a Human

- **Method**: `DELETE`
- **Endpoint**: `/human/{id}`

## 🤝 Contribute

Feel free to fork the project, make some updates, and submit pull requests.

## 📄 License

This project is licensed under the MIT License.
# rusting-actix-diesel
