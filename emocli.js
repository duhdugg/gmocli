#!/usr/bin/env node

const path = require('path')
const util = require('util')

const gitmojiPath = path.join(
  __dirname,
  'gitmoji',
  'src',
  'data',
  'gitmojis.json'
)
const gitmojis = require(gitmojiPath).gitmojis

// const unicodeEmojiPath = ~/Projects/personal/emocli/unicode-emoji-json/data-by-emoji.json
const unicodeEmojiPath = path.join(
  __dirname,
  'unicode-emoji-json',
  'data-by-emoji.json'
)
const emojiData = require(unicodeEmojiPath)
const emojis = Object.keys(emojiData)

for (const emoji of emojis) {
  for (const gitmoji of gitmojis) {
    if (emoji === gitmoji.emoji) {
      Object.assign(emojiData[emoji], {
        gitmojiName: gitmoji.name,
        gitmojiCode: gitmoji.code,
        gitmojiDesc: gitmoji.description
      })
    }
  }
}

function printHelp() {
  const lines = [
    `Usage:\tgitmocli [OPTIONS] <search>`,
    '',
    'OPTIONS:',
    '-h \t print this help',
    '-l \t list all emojis',
    '-d \t include data',
    '-1 \t output the first result only'
  ]
  console.log(lines.join('\n'))
}

function printList(withData = false) {
  const lines = []
  for (const emoji of emojis) {
    printEmoji(emoji, withData)
  }
}

function printEmoji(emoji, withData = false) {
  if (withData) {
    const data = emojiData[emoji]
    console.log(
      emoji,
      util.inspect(data, { breakLength: Infinity, colors: true })
    )
  } else {
    process.stdout.write(emoji)
  }
}

function searchEmojis(searchKeys) {
  const matches = []
  for (let emoji of emojis) {
    const data = emojiData[emoji]
    let match = true
    // search name
    for (let searchKey of searchKeys) {
      if (!data.name.toLowerCase().includes(searchKey.toLowerCase())) {
        match = false
        break
      }
    }
    if (!match && data.gitmojiName) {
      match = true
      for (let searchKey of searchKeys) {
        if (!data.gitmojiName.toLowerCase().includes(searchKey.toLowerCase())) {
          match = false
          break
        }
      }
    }
    if (!match && data.gitmojiCode) {
      match = true
      for (let searchKey of searchKeys) {
        if (!data.gitmojiCode.toLowerCase().includes(searchKey.toLowerCase())) {
          match = false
          break
        }
      }
    }
    if (!match && data.gitmojiDesc) {
      match = true
      for (let searchKey of searchKeys) {
        if (!data.gitmojiDesc.toLowerCase().includes(searchKey.toLowerCase())) {
          match = false
          break
        }
      }
    }
    if (match) {
      matches.push(emoji)
    }
  }
  return matches
}

if (process.argv.length < 3) {
  printHelp()
} else {
  const helpFlag = process.argv.includes('-h')
  const listFlag = process.argv.includes('-l')
  const dataFlag = process.argv.includes('-d')
  const snglFlag = process.argv.includes('-1')
  if (helpFlag) {
    printHelp()
  }
  if (listFlag) {
    printList(dataFlag)
  }

  const searchKeys = process.argv
    .slice(2)
    .filter((arg) => !['-h', '-l', '-d', '-1'].includes(arg))
  for (let emoji of searchEmojis(searchKeys)) {
    printEmoji(emoji, dataFlag)
    if (snglFlag) {
      break
    }
  }
}
