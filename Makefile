setup:
	cargo install diesel_cli --no-default-features --features sqlite

setup-ubuntu:
	sudo apt install libsqlite3-dev -y
	make setup

setup-arch:
	sudo pacman -S sqlite
	make setup

setup-nixos: 
	nix-shell

copy_env_template:
	cp --backup templates/env.template .env

copy_schedule_template:
	cp --backup templates/import_schedule.json.template import_schedule.json

test: setup
	DATABASE_URL=test_gateway_garden.sqlite sh -c 'cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Xml' && rm test_gateway_garden.sqlite

