#!/bin/bash

#当前时间 对应的毫秒时间戳

current=$(date +%Y%m%d%H%M%S)
echo "$current"
#timeStamp=date -d "$current" +%s  
#将current转换为时间戳，精确到毫秒  
#currentTimeStamp=$((timeStamp*1000+`date "+%N"`/1000000)) 
#echo $currentTimeStamp

# cd /Users/shaipe/workspace/ecdata/ui-admin

# yarn build 

# zip -r dist.zip ./dist/

# scp ./dist.zip root@192.168.17.213:/srv/docker/nginx/html

# rm -rf dist.zip
