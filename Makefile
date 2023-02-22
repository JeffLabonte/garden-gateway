setup:
	cargo install diesel_cli --no-default-features --features sqlite

setup-test:
	cargo install cargo-tarpaulin

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

test: setup setup-test
	DATABASE_URL=test_gateway_garden.sqlite sh -c 'cargo tarpaulin --all-features --workspace --timeout 120 --out Xml -- --test-threads=1' && rm test_gateway_garden.sqlite