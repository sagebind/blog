.PHONY: build
build:
	docker-compose build

.PHONY: run
run: build
	docker-compose run --rm -p 5000:80 app

.PHONY: css
css: wwwroot/assets/css/style.min.css

wwwroot/assets/css/style.min.css: styles/*.scss
	sassc --style compressed styles/base.scss $@
