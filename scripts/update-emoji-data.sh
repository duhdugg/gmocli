#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd $SCRIPT_DIR/..

EMOJI_DATA_VERSION="2.4"
GITMOJI_VERSION="3.9.0"
EMOJI_URL="https://github.com/Mange/emoji-data/releases/download/v$EMOJI_DATA_VERSION/prebuilt-data-v$EMOJI_DATA_VERSION.tar.bz2"
GITMOJI_URL="https://github.com/carloscuesta/gitmoji/archive/refs/tags/v$GITMOJI_VERSION.tar.gz"

mkdir -p data/emojis data/gitmojis dev/emoji-data-dist dev/gitmoji

cd dev/emoji-data-dist
curl -L "$EMOJI_URL" > prebuilt-data.tar.bz2
tar -xf prebuilt-data.tar.bz2
cat data/all_emojis.json | jq -c  > ../../data/emojis/emojis.json

cd ../gitmoji
curl -L "$GITMOJI_URL" > gitmoji.tar.gz
tar -xf gitmoji.tar.gz
cat "gitmoji-$GITMOJI_VERSION/src/data/gitmojis.json" | jq -c > ../../data/gitmojis/gitmojis.json
