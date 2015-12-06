FROM php:7-cli
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apt-get update \
    && apt-get install zlib1g-dev ruby-sass git -y \
    && docker-php-ext-install zip \
    && docker-php-ext-install mbstring

RUN mkdir -p /app/static

ADD bin             /app/bin
ADD src             /app/src
ADD static/assets   /app/static/assets
ADD styles          /app/styles
ADD templates       /app/templates
ADD vendor          /app/vendor
ADD composer.json   /app/composer.json
ADD composer.lock   /app/composer.lock
ADD RoboFile.php    /app/RoboFile.php

WORKDIR /app
RUN curl -sS https://getcomposer.org/installer | php \
    && php composer.phar install \
    && vendor/bin/robo build

VOLUME ["/app/articles", "/app/static/content"]

EXPOSE 8080

ENTRYPOINT ["/app/bin/server"]
