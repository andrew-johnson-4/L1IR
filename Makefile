test_active_branch:
	cargo test --features cranelift eval_flatmap1 -- --nocapture

test_all:
	cargo test --features cranelift

bench:
	cargo build --release --features cranelift
	./target/release/bench
	ocamlopt -o target/bench_ml benches/main.ml
	time ./target/bench_ml

next:
	cargo test eval_match1 --features cranelift -- --nocapture
