FROM php:7-alpine
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apk add --no-cache git ruby ruby-irb ruby-rdoc \
    && docker-php-ext-install sockets \
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

EXPOSE 80

CMD ["/app/bin/server"]
