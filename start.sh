#!/bin/sh
service appserver start
service appserver-php5-fpm start

# Run forever unless appserver crashes
PID=`pgrep -o php`
while kill -0 "$PID"; do
    sleep 1
done
