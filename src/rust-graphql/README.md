<div align="center">
  <h1>Rust GraphQL</h1>

<img src='docs/construction.svg' width=80px />

Rust GraphQL Template 🏗️

<a href="https://github.com/azzamsa/rust-graphql/workflows/ci.yml">
    <img src="https://github.com/azzamsa/rust-graphql/workflows/ci/badge.svg" alt="Build status" />
  </a>

</div>

---

## Features

- [Async-GraphQL](https://github.com/async-graphql/async-graphql) GraphQL server library
  - Relay compatible cursor-based pagination
  - Playground disabled in the production environment for performance reasons
  - Store GraphQL schema to file automatically
- [SQLx](https://github.com/launchbadge/sqlx) SQL toolkit
- [Axum](https://github.com/tokio-rs/axum) web framework
- [utoipa](https://github.com/juhaku/utoipa) Auto-generated OpenAPI documentation
- [git-cliff](https://github.com/orhun/git-cliff) Changelog Generator
- Exhaustive Integration tests
  - Uses [Cynic](https://github.com/obmarg/cynic) for GraphQL client
- Github Action for CI and release
- Git hooks for continuous development (format, lint, test)
  - Uses [Cargo Husky](https://github.com/rhysd/cargo-husky)
- Consistent formatting using [dprint](https://github.com/dprint/dprint)

## Navigating the Code

All the features can be found in the [CHANGELOG](CHANGELOG.md) file tagged with `feat`.
The file only contains user-facing changes, so you won't get lost navigating the code.

## Usage

```shell
$ # Clone the repository

$ # Create a database
$ sudo -u postgres createdb graphql

$ touch $SCHEMA_LOCATION # See .example.env
$ make dev  # See also `make setup`
```

Go to the playground `http://127.0.0.1:8000/playground` to see the schema.

## Credits

- [Clean and Scalable Architecture for Web Applications in Rust](https://kerkour.com/rust-web-application-clean-architecture) by Sylvain Kerkour
- Icons and emoji from [Noto Emoji](https://github.com/googlefonts/noto-emoji)
