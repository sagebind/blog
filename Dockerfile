FROM php:7-cli
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apt-get update \
    && apt-get install zlib1g-dev ruby-sass git -y \
    && docker-php-ext-install zip \
    && docker-php-ext-install mbstring

ADD articles        /app/articles
ADD bin             /app/bin
ADD src             /app/src
ADD static          /app/static
ADD styles          /app/styles
ADD templates       /app/templates
ADD composer.json   /app/composer.json
ADD composer.lock   /app/composer.lock
ADD RoboFile.php    /app/RoboFile.php

WORKDIR /app
RUN curl -sS https://getcomposer.org/installer | php \
    && php composer.phar install \
    && vendor/bin/robo build

EXPOSE 8080

ENTRYPOINT ["/app/bin/server"]
