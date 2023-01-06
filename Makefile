test_all:
	cargo test --features cranelift

bench:
	cargo build --features cranelift
	./target/debug/bench
