FROM php:7-cli
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apt-get update \
    && apt-get install zlib1g-dev -y \
    && docker-php-ext-install zip \
    && docker-php-ext-install mbstring

RUN mkdir -p /app/static

ADD bin             /app/bin
ADD src             /app/src
ADD static/assets   /app/static/assets
ADD templates       /app/templates
ADD composer.json   /app/composer.json
ADD composer.lock   /app/composer.lock

RUN cd /app \
    && curl -sS https://getcomposer.org/installer | php \
    && php composer.phar install

VOLUME ["/app/articles", "/app/static/content"]

EXPOSE 8080

ENTRYPOINT ["/app/bin/server"]
