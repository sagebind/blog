FROM php:7-alpine
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apk add --no-cache ruby ruby-irb ruby-rdoc \
    && gem install sass

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

EXPOSE 8000

ENTRYPOINT ["/app/bin/server"]