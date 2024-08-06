default: all

all: demo

wasm_rust_flags='-C target-feature=+simd128,+bulk-memory,+multivalue'
wasm_cargo_flags=--release --target wasm32-unknown-unknown
wasm_release_dir=target/wasm32-unknown-unknown/release
wasm_opt_flags=-all -O4 --strip-debug --strip-dwarf --strip-producers --strip-target-features

wasm-build:
	RUSTFLAGS=$(wasm_rust_flags) cargo build $(wasm_cargo_flags)

demo: wasm-build
	mkdir -p demo/web
	wasm-opt $(wasm_release_dir)/demo.wasm $(wasm_opt_flags) -o demo/web/bin.wasm
	wasm2wat demo/web/bin.wasm -o demo/web/bin.wat -f --inline-exports --enable-all
	ls -l demo/web