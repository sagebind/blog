FROM debian:8
MAINTAINER Stephen Coakley <me@stephencoakley.com>

RUN apt-get update && apt-get install -y \
    curl \
    php5-cli \
    php5-fpm

# Install appserver.io
RUN echo "deb http://deb.appserver.io/ jessie main" > /etc/apt/sources.list.d/appserver.list && \
    curl http://deb.appserver.io/appserver.gpg -s -S | apt-key add -
RUN apt-get update && apt-get install -y appserver-dist; \
    chmod +x /etc/init.d/appserver*

# Copy website to appserver directory
ADD appserver.xml /opt/appserver/etc/appserver/appserver.xml
ADD . /opt/appserver/webapps/blog/

EXPOSE 80

# Create a startup script to start the web server
ADD start.sh /opt/appserver/start.sh
RUN chmod +x /opt/appserver/start.sh
CMD ["/opt/appserver/start.sh"]

# Clean up extra space
RUN apt-get clean && rm -rf /tmp/* /var/tmp/*
