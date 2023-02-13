test_all:
	cargo test bug_nested_flatmap --features cranelift -- --nocapture

test_one:
	cargo test bug_nested_flatmap --features cranelift -- --nocapture

bench:
	cargo build --release --features cranelift
	./target/release/bench
	ocamlopt -o target/bench_ml benches/main.ml
	time ./target/bench_ml

next:
	cargo test eval_match1 --features cranelift -- --nocapture
