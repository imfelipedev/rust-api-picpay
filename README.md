# 🚩 Authentication API

This application was developed to solve a [PicPay Backend Challenge](https://github.com/PicPay/picpay-desafio-backend) using Rust and DDD architecture.

Main libraries used:

-   Tokio;
-   Axum;
-   SQLx;
-   Serde;
-   Reqwest;

Project structure:

```
.
├── cargo.toml
└── src
    ├── controllers
    │   ├── transaction_controller.rs
    │   └── user_controller.rs
    ├── domain
    │   ├── appstate
    │   │   └── appstate.rs
    │   ├── authorizer
    │   │   └── authorizer.rs
    │   ├── notification
    │   │   └── notification.rs
    │   ├── transaction
    │   │   └── transaction.rs
    │   └── user
    │       ├── user.rs
    │       └── user_query.rs
    ├── infrastructure
    │   ├── database.rs
    │   └── router.rs
    ├── repositories
    │   └── user_repository.rs
    ├── services
    │   ├── authorizer_service.rs
    │   ├── notification_service.rs
    │   ├── transaction_service.rs
    │   └── user_service.rs
    ├── lib.rs
    └── main.rs
```

## Installation

1. Download and install Rust on your machine: https://www.rust-lang.org/pt-BR/learn/get-started
2. Clone repository.
3. Configure the `.env` file.

## Running in development.

Running migrate:

```bash
sqlx migrate run
```

Start server:

```bash
cargo run
```

## Deploy

Build project:

```bash
cargo build --release
```

Start the executable file:

```
target/release/rust-api-picpay.exe
```

## API Endpoints

#### Register a new user.

```http
POST /api/v1/user/register
```

| Parâmetro        | Tipo     | Descrição                |
| :--------------- | :------- | :----------------------- |
| `name`           | `string` | User's full name         |
| `identification` | `string` | User CPF/CNPJ            |
| `email`          | `string` | User email               |
| `password`       | `string` | User password            |
| `user_type`      | `string` | `Customer` or `Merchant` |

#### Create a new transaction between users.

```http
POST /api/v1/transaction/create
```

| Parâmetro | Tipo      | Descrição         |
| :-------- | :-------- | :---------------- |
| `value`   | `integer` | Transaction value |
| `payer`   | `integer` | Payer user ID     |
| `payee`   | `integer` | Payee user ID     |

## Reference

-   [PicPay Backend Challenge](https://github.com/PicPay/picpay-desafio-backend?tab=readme-ov-file)

## Autores

-   [@zFelipeA](https://github.com/zFelipeA)
