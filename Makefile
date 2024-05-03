run:
	cargo build
	sudo chown root:root target/debug/dosu
	sudo chmod u+s target/debug/dosu
	./target/debug/dosu $(cmd)

build:
	cargo build --release

install:
	make build
	sudo cp target/release/dosu /usr/local/bin/dosu
	sudo chown root:root /usr/local/bin/dosu
	sudo chmod u+s /usr/local/bin/dosu