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
const unicodeEmojiData = require(unicodeEmojiPath)
const emojis = Object.keys(unicodeEmojiData)
const emojiData = {}

for (const emoji of emojis) {
  emojiData[emoji] = {
    name: unicodeEmojiData[emoji].name
  }
  for (const gitmoji of gitmojis) {
    if (emoji === gitmoji.emoji) {
      Object.assign(emojiData[emoji], {
        gitmojiDesc: gitmoji.description.toLowerCase()
      })
    }
  }
}

function printHelp() {
  const lines = [
    `Usage:\tgitmocli [OPTIONS] <search>`,
    '',
    'OPTIONS:',
    '-h | --help \t print this help',
    '-l | --list \t list all emojis',
    '-i | --info \t include info',
    '-n | --name \t match name exactly'
  ]
  console.log(lines.join('\n'))
}

function printList(withInfo = false) {
  const lines = []
  for (const emoji of emojis) {
    printEmoji(emoji, withInfo)
  }
}

function printEmoji(emoji, withInfo = false) {
  if (withInfo) {
    const data = emojiData[emoji]
    console.log(
      emoji,
      `${data.name}${data.gitmojiDesc ? ' / ' + data.gitmojiDesc : ''}`
    )
  } else {
    process.stdout.write(emoji)
  }
}

function searchEmojis(searchKeys) {
  const matches = []
  if (!searchKeys.length) {
    return matches
  }
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

function getEmojiByName(name) {
  let match
  for (let emoji of emojis) {
    const data = emojiData[emoji]
    if (data.name === name) {
      match = emoji
      break
    }
  }
  return match
}

const args = process.argv
if (args.length < 3) {
  printHelp()
} else {
  let helpFlag = args.includes('-h') || args.includes('--help')
  let listFlag = args.includes('-l') || args.includes('--list')
  let infoFlag = args.includes('-i') || args.includes('--info')
  let nameFlag = args.includes('-n') || args.includes('--name')
  for (let arg of args) {
    if (arg[0] === '-' && arg[1] !== '-') {
      if (arg.includes('h')) {
        helpFlag = true
      }
      if (arg.includes('l')) {
        listFlag = true
      }
      if (arg.includes('i')) {
        infoFlag = true
      }
      if (arg.includes('n')) {
        nameFlag = true
      }
    }
  }
  if (helpFlag) {
    printHelp()
  }
  if (listFlag) {
    printList(infoFlag)
  }
  const searchKeys = args.slice(2).filter((arg) => !arg.match(/^-/))

  if (nameFlag) {
    const match = getEmojiByName(searchKeys.join(' '))
    if (match !== undefined) {
      printEmoji(match, infoFlag)
    } else {
      process.exit(1)
    }
  } else {
    const matches = searchEmojis(searchKeys)
    if (matches.length) {
      for (let emoji of matches) {
        printEmoji(emoji, infoFlag)
      }
    } else {
      process.exit(1)
    }
  }
}
