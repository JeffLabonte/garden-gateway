SHELL := /bin/bash

install:
	poetry install

install-rpi: install
	sudo apt install python3-rpi.gpio
