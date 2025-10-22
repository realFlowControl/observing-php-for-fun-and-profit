.PHONY: test build prep

shell:
	docker compose run --rm dev bash

prep:
	docker compose build
	docker compose pull
	docker compose run --rm --remove-orphans dev make

build:
	docker compose run --rm --remove-orphans dev make

test:
	docker compose run --rm --remove-orphans dev make test
