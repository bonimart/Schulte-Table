.PHONY: all linux android clean

linux:
	cargo run

android:
	cargo apk run --lib

clean:
	rm -rf target
	cargo clean
