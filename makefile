run:
	# 0、auto open swaggerui website
	@open http://127.0.0.1:3000/swagger-ui/
	# 1、run axum app
	@cargo run --bin axum_app

migration:
	# apply sql to database
	@sqlx migrate run

docker:
	# rebuild axum_app image and restart docker container
	@docker-compose build axum_app && docker-compose up -d    