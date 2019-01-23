DATA_FILE := ~/Downloads/test_accounts_291218/data/data.zip

.PHONY: start linux docker local-docker push clean

start:
	cargo build
	rm -rf './tmp'
	sh ./bin/start.sh './tmp' $(DATA_FILE) target/debug/hlc-rs

linux:
	docker build -t hlc-rs-builder --file ./tools/builder/Dockerfile .
	mkdir -p ./build
	docker cp $$(docker create hlc-rs-builder):/build/target/x86_64-unknown-linux-musl/release/hlc-rs ./build/linux

docker: linux
	docker build -t hlc-rs .

local-docker: docker
	cp $(DATA_FILE) ./harness/data.zip
	docker build -t hlc-rs-local ./harness/
	docker run -p 8080:80 -ti hlc-rs-local

push: docker
	docker tag hlc-rs stor.highloadcup.ru/accounts/electric_panda
	docker push stor.highloadcup.ru/accounts/electric_panda

clean:
	rm -rf ./build
	cargo clean
