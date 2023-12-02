all: test

test: test.rs
	mkdir -p bin
	rustc -o bin/test test.rs

clean:
	rm -rf bin/*