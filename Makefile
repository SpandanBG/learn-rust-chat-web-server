# Variables
SRC_DIR = src
BUILD_DIR = target
LIGHTHOUSE_FILE = localhost.report.html


# Determine the OS
ifeq ($(OS),Windows_NT)
	RM = del /Q
	RM_DIR = rmdir /S /Q
else
	RM = rm -rf
	RM_DIR = rm -rf
endif

# Targets
build:
	cargo build

release:
	cargo build --release

run:
	cargo run

run-release:
	cargo run --release

watch:
	cargo watch -w $(SRC_DIR) -x run

clean:
	$(RM_DIR) $(TARGET_DIR)

lighthouse:
	lighthouse http://localhost:8080 --view --output-path=./$(LIGHTHOUSE_FILE)

clean-lighthouse:
	$(RM) $(LIGHTHOUSE_FILE) 