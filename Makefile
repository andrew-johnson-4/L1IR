test_all:
	cargo test --features cranelift --test b_opt3 -- --nocapture

bench:
	cargo build --features cranelift
	./target/debug/bench
	ocamlopt -o target/bench_ml benches/main.ml
	time ./target/bench_ml

next:
	cargo test eval_match1 --features cranelift -- --nocapture
