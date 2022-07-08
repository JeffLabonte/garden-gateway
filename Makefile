setup:
	sudo apt install libsqlite3-dev -y
	cargo install diesel_cli --no-default-features --features sqlite

copy_env_template:
	cp --backup templates/env.template .env

copy_schedule_template:
	cp --backup templates/import_schedule.json.template import_schedule.json

test: setup
	DATABASE_URL=test_gateway_garden.sqlite sh -c "diesel migration run && diesel migration redo && cargo test"
