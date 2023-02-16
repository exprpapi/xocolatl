.PHONY: install
install:
	npm install
	npm run build

.PHONY: serve
serve: install
	npm run serve

.PHONY: clean
clean:
	rm -rf node_modules
	rm -rf target
	rm -rf pkg
	rm -rf www
	rm Cargo.lock
	rm package-lock.json

.PHONY: test
test:
	cargo test
