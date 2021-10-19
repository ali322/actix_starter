# Axum Starter
> API service starter project with actix + sea-orm

[![License][license-image]][license-url]

### Prepare database

- install migration tool via `cargo install sqlx-cli`
- specify your database connection from `DATABASE_URL` of `.env` file 
- run migrations via `sqlx migrate run`
- generate entites via `sea-orm-cli generate entity -o src/repository/entity`

[license-image]: https://img.shields.io/badge/License-MIT-blue.svg
[license-url]: https://vsouza.mit-license.org