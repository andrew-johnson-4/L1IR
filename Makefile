test_all:
	cargo test --features cranelift -- --nocapture

bench:
	cargo build --features cranelift
	./target/debug/bench
	ocamlopt -o target/bench_ml benches/main.ml
	time ./target/bench_ml
