all: boxes rc

boxes: boxes.rs
	rustc boxes.rs
rc: rc.rs
	rustc rc.rs

.PHONY: clean

clean:
	rm -rf boxes rc
