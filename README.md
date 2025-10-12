![CI](https://github.com/dhcsousa/hospitopt-rs/actions/workflows/ci.yml/badge.svg)

# hospitopt-rs

Rust-based optimization project that uses constraint programming to maximize the number of lives saved in emergency and healthcare scenarios. It models hospitals, diseases, available beds, ambulance positions and capacities, and patient needs, then computes optimized resource allocations to improve medical response and outcomes.

## Vision

- Capture hospitals, treatments, bed capacity, ambulance fleets, and patient demand in a unified Rust model.
- Explore Google OR-Tools CP-SAT (via the `cp_sat` bindings) to optimize triage, routing, and resource allocation.

## Getting started

1. Start your dotenv file and modify it accordingly, including your Google API key

	```shell
	cp .env.example .env
	```

2. Start PostgreSQL locally with Docker Compose (runs on `localhost:5432`)

	```shell
	docker compose up -d postgres
	```

3. Apply database migrations

	```shell
	sea-orm-cli migrate up
	```

	> Need a clean slate? Use `sea-orm-cli migrate refresh`.

4. Run the tests or binaries as usual

	```shell
	cargo test
	```

	```shell
	cargo run
	```

5. Seed curated reference data (optional)

	```shell
	cargo run -p scripts
	```

	This command clears the existing `hospitals` and `hospital_specialities` tables and inserts a curated dataset covering eight flagship hospitals across Lisbon. It uses coordinates sourced from the public GeoJSON dataset at [dados.gov.pt](https://dados.gov.pt/pt/datasets/r/214f62f9-ff13-48d2-ae80-c33879c441fa) and fills in illustrative bed capacities plus Manchester triage wait targets so you can experiment with the optimizer immediately.

	Need to add more synthetic patients without wiping the previous ones? Set the mode to `append` when running the script:

	```shell
	PATIENT_SEED_MODE=append cargo run -p scripts
	```

	The default mode is `reset`, which keeps the deterministic hospital snapshot while refreshing patients on every run.

When you're done developing, stop the database container with `docker compose down` (add `-v` to prune the named volume).

## Google API for Distance Matrix

Since there are no officially supported Rust crates for the [Google Maps Routes API](https://developers.google.com/maps/documentation/routes), this project ships a lightweight client located at `src/google_maps/routes.rs`. If you need help setting up a Google Cloud project and enabling the Routes API, check out [this guide](https://developers.google.com/maps/documentation/routes/get-api-key).

Add your API key to the environment (for example in `.env`):

```dotenv
GOOGLE_API_KEY=your_api_key
```
