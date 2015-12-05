FROM php:7-cli
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN docker-php-ext-configure mbstring \
    && docker-php-ext-install mbstring \
    && docker-php-ext-configure zip \
    && docker-php-ext-install zip

RUN mkdir /app

ADD articles        /app/articles
ADD bin             /app/bin
ADD src             /app/src
ADD static          /app/static
ADD templates       /app/templates
ADD composer.json   /app/composer.json
ADD composer.lock   /app/composer.lock

RUN cd /app \
    && curl -sS https://getcomposer.org/installer | php \
    && php composer.phar install

EXPOSE 8080

ENTRYPOINT ["/app/bin/server"]
