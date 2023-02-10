test_all:
	cargo test --features cranelift

bench:
	cargo build --release --features cranelift
	./target/release/bench
	ocamlopt -o target/bench_ml benches/main.ml
	time ./target/bench_ml

next:
	cargo test eval_match1 --features cranelift -- --nocapture
