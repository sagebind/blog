FROM php:7-cli
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN docker-php-ext-configure mbstring \
    && docker-php-ext-install mbstring

RUN mkdir /app

ADD articles    /app/articles
ADD bin         /app/bin
ADD src         /app/src
ADD static      /app/static
ADD templates   /app/templates
ADD vendor      /app/vendor

EXPOSE 8080

ENTRYPOINT ["/app/bin/server"]
