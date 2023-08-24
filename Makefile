run-docker: close-docker-apps postgres

postgres:
	docker run --name grpc-demo-db \
	-p 5432:5432 -e POSTGRES_PASSWORD=grpc \
	-e POSTGRES_USER=grpc-user -e POSTGRES_DB=grpc-demo-db \
	-d postgres

close-docker-apps: close-postgres

close-postgres:
	docker rm -f grpc-demo-db

dev: run-docker
	cargo watch -x run