SHELL := /bin/bash

install:
	poetry install

install-rpi: install
	sudo apt install python3-rpi.gpio

service:
	@echo "Will create service":
	sudo cp scripts/service/garden-gateway.service /etc/systemd/system
	sudo systemctl daemon-reload
	sudo systemctl enable --now garden-gateway.service
