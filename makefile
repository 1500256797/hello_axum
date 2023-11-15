run:
	# 0、auto open swaggerui website
	@open http://127.0.0.1:3000/swagger-ui/
	# 1、run axum app
	@cargo run --bin axum_app

migration:
	# 2、apply sql to database
	@sqlx migrate run