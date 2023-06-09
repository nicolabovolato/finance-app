# Personal finance app

[![Api CI](https://github.com/nicola-bovolato/finance-app/actions/workflows/api.yaml/badge.svg)](https://github.com/nicola-bovolato/finance-app/actions/workflows/api.yaml)
[![Web CI](https://github.com/nicola-bovolato/finance-app/actions/workflows/web.yaml/badge.svg)](https://github.com/nicola-bovolato/finance-app/actions/workflows/web.yaml)

A simple finance app built with with SvelteKit and Rust.

<div style="display:flex">
    <img width="49.5%" src="img-overview.png" />
    <img width="49.5%" src="img-business-account.png" />
</div>

### Run the app

- `docker compose up -d`
- Visit [localhost](http://localhost)
- Visit [mailcatcher](http://localhost:1080) for your OTPs

## Technologies

### [Frontend](./web/)

- `Sveltekit`
- `Typescript`
- `Tanstack Query`
- `Svelte Forms Lib`
- `Tailwind`
- `Frappe Charts`
- `Heroicons`

### [Backend](./api/)

- `Rust`
- `Axum`
- `Sqlx`
- `Paseto`
- `Mockall`
- `Postgres`
- `Redis`
- `SMTP`

### Putting it all together

- `Docker`
