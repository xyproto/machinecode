PREFIX ?= /usr/local
DESTDIR ?=
BIN_NAME = jitrun

BIN_DIR = $(DESTDIR)$(PREFIX)/bin

.PHONY: all install clean

all:
	cargo build --release

install: all
	mkdir -p $(BIN_DIR)
	install target/release/$(BIN_NAME) $(BIN_DIR)/$(BIN_NAME)

clean:
	cargo clean
