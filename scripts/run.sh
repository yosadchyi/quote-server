#!/bin/sh

echo Starting server...
target/release/quote-server 127.0.0.1:8080 &>/tmp/server.log &
echo Waiting for server...
sleep 3
echo Running client
target/release/quote-client 127.0.0.1:8080 10
