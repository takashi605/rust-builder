.PHONY: test test-down

test:
	docker compose --profile test up --build

test-down:
	docker compose --profile test down
