# Variables
SRC_DIR = src
BUILD_DIR = target

build:
	cargo build

run:
	cargo run

watch:
	cargo watch -w $(SRC_DIR) -x run

lighthouse:
	lighthouse http://localhost:8080 --view --output-path=./localhost.report.html