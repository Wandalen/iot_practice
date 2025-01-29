.PHONY: up down

up:
	docker compose --env-file ./secret/-env.sh up -d --build --force-recreate

down:
	docker compose down -v
