setup:
	cargo install diesel_cli --no-default-features --features sqlite

copy_env_template:
	cp template/env.template .env

copy_schedule_template:
	cp template/env.template .env
