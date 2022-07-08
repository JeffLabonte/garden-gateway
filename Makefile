setup:
	cargo install diesel_cli --no-default-features --features sqlite

copy_env_template:
	cp --backup templates/env.template .env

copy_schedule_template:
	cp --backup templates/import_schedule.json.template import_schedule.json

test:
	DATABASE_URL=test_gateway_garden.sqlite diesel migration redo
	DATABASE_URL=test_gateway_garden.sqlite cargo test
