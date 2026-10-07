#!/bin/sh
set -eu

# Run on licheerv-nano; stop the foreground server with Ctrl-C.
cd /root/sw-apl
test -x ./sw-apl-server
test -d ./ws/lib2
printf 'Starting APL mode 75 on licheerv-nano.\n'
printf 'Connect from your computer:\n  aplterm --connect 10.105.250.1:2775\n'
printf 'Keep this session open; Ctrl-C stops this server.\n'
exec ./sw-apl-server \
    --mode 75 \
    --listen 10.105.250.1:2775 \
    --http 127.0.0.1:8375 \
    --library /root/sw-apl \
    --lib 2=/root/sw-apl/ws/lib2,EXTENDED \
    --sessions 2
