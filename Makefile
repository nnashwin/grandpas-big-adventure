.PHONY: dev
dev:
	cargo watch -x run

.PHONY: test
test:
	cargo test
