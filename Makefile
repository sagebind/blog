SCSS_STYLE = compressed


all: php css

clean:
	rm -rf static/assets/css vendor

serve: all
	env ENVIRONMENT=LOCAL php bin/server

php: vendor/autoload.php

vendor/autoload.php: composer.phar
	php composer.phar install

composer.phar:
	curl -L http://getcomposer.org/composer.phar > composer.phar

css: static/assets/css
	sass --style $(SCSS_STYLE) --trace styles/base.scss static/assets/css/style.css
	postcss --use autoprefixer --replace css static/assets/css/style.css

static/assets/css:
	mkdir -p $@

