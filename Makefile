SHELL := /bin/bash

all:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-pack --force
	wasm-pack build --target browser