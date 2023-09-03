.POSIX:

VERSION=1.0.0
PREFIX=/usr/local
MANPREFIX=$(PREFIX)/share/man

SRC_DIR=$(shell pwd)/src
SRCS=$(shell find $(SRC_DIR) -name '*.rs')
TARGET=target/release/ucpu

all: $(TARGET)

$(TARGET): $(SRCS) Cargo.lock
	cargo build --release

clean:
	cargo clean

install: $(TARGET)
	mkdir -p $(PREFIX)/bin
	cp -f $(TARGET) $(PREFIX)/bin
	chmod 755 $(PREFIX)/bin/ucpu
	mkdir -p $(MANPREFIX)/man1
	cp -f ucpu.1 $(MANPREFIX)/man1/ucpu.1
	gzip -f $(MANPREFIX)/man1/ucpu.1

uninstall:
	rm -f $(PREFIX)/bin/ucpu
	rm -f $(MANPREFIX)/man1/ucpu.1.gz

.PHONY: all clean install uninstall
