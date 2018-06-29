build:
	docker-compose build

run: build
	docker-compose run --rm -p 5000:80 app
