## Pre-requisite
You'll need to install:
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

## How to build
Using `cargo`:

```bash
cargo build
```

## How to test
Using `cargo`:

```bash
cargo test
```

## Create DB
`sqlx database create`
## Add migrations
`sqlx migrate add create_subscriptions_table`

## Run migrations
`SKIP_DOCKER=true ./scripts/init_db.sh`

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>