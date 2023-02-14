test_all:
	cargo test -- --nocapture

test_one:
	cargo test bug_nested_flatmap -- --nocapture

bench:
	cargo build --release
	./target/release/bench
	ocamlopt -o target/bench_ml benches/main.ml
	time ./target/bench_ml
