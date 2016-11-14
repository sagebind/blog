FROM php:7-alpine
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apk add --no-cache \
        git \
        make \
        nodejs \
        ruby \
        ruby-irb \
        ruby-rdoc && \
    docker-php-ext-install sockets && \
    gem install sass && \
    npm install --global postcss-cli autoprefixer

ADD articles        /app/articles
ADD bin             /app/bin
ADD src             /app/src
ADD static          /app/static
ADD styles          /app/styles
ADD templates       /app/templates
ADD composer.json   /app/composer.json
ADD composer.lock   /app/composer.lock
ADD Makefile        /app/Makefile

RUN make -C /app all

EXPOSE 80

CMD ["/app/bin/server"]
