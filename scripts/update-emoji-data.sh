#!/usr/bin/env bash

cd dev
mkdir -p emoji-data-dist
cd emoji-data-dist
wget https://github.com/Mange/emoji-data/releases/download/v2.3/prebuilt-data-v2.3.tar.bz2
tar -xf prebuilt-data-v2.3.tar.bz2
cat data/all_emojis.json | jq -c | gzip > ../../data/emojis.json.gz
