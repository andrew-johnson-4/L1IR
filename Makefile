test_all:
	cargo test --features cranelift

bench:
	cargo build --release --features cranelift
	./target/release/bench
