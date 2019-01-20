.PHONY: linux docker local-docker push clean

linux:
	docker build -t hlc-rs-builder --file ./tools/builder/Dockerfile .
	docker create --name hlc-rs-builder-cont hlc-rs-builder
	mkdir -p ./build
	docker cp hlc-rs-builder-cont:/target/release/hlc-rs ./build/linux

docker: linux
	docker build -t hlc-rs .

local-docker: docker
	docker build -t hlc-rs-local ./harness/
	docker run -p 8080:80 -ti hlc-rs-local

push: docker
	docker tag hlc-rs stor.highloadcup.ru/accounts/electric_panda
	docker push stor.highloadcup.ru/accounts/electric_panda

clean:
	rm -rf ./build
	cargo clean
