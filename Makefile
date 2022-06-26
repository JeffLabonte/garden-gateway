setup:
	cargo install diesel_cli --no-default-features --features sqlite

copy_template:
	cp env.template .env
