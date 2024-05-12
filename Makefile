build-prod-app:
	docker buildx build -t idioms-api:latest .
run-db:
	sudo docker compose -f docker-compose.db.yml up --remove-orphans
stop-db:
	sudo docker compose -f docker-compose.db.yml down
build-tar:
	docker save -o idiomsapi.tar idioms-api
