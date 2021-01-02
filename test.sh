#!/bin/bash

cd /Users/shaipe/workspace/ecdata/ui-admin

yarn build 

zip -r dist.zip ./dist/

scp ./dist.zip root@192.168.17.213:/srv/docker/nginx/html

rm -rf dist.zip
