# emocli

emocli provides a command-line interface for searching emoji characters.

It uses combined data from [Mange/emoji-data](https://github.com/Mange/emoji-data) and [carloscuesta/gitmoji](https://github.com/carloscuesta/gitmoji).

**Homepage:** https://github.com/duhdugg/emocli

## Usage

```text
Usage:	emocli [OPTIONS] <search>

OPTIONS:
-h | --help 	print this help
-l | --list 	list all emoji characters
-i | --info 	include info
-n | --name 	match name exactly
```

### Examples

#### list all emoji characters with their name, info, and gitmoji description (if available)

`emocli -li`

#### search for an emoji using keywords

```bash
emocli -i dog
# output
🐶 dog face | Animals & Nature / animal-mammal | dog,face,pet
🐕 dog | Animals & Nature / animal-mammal | dog,pet
🦮 guide dog | Animals & Nature / animal-mammal | accessibility,blind,guide,guide dog
🐕‍🦺 service dog | Animals & Nature / animal-mammal | accessibility,assistance,dog,service
🐩 poodle | Animals & Nature / animal-mammal | dog,poodle
🌭 hot dog | Food & Drink / food-prepared | frankfurter,hot dog,hotdog,sausage
```

#### print a specific emoji by name

if the name is more than one word, it should be enclosed in quotes

```bash
emocli -n 'guide dog'
# output
🦮
```

#### use in a commit message

`git commit -m "$(emocli -n rocket) production launch"`

#### use with rofi and xclip

[rofi](https://github.com/davatorium/rofi) is a window switcher, application launcher, and dmenu replacement. Here, it is being used as a dmenu replacement to provide a graphical interface for search and select.

[xclip](https://github.com/astrand/xclip) is a command line utility that provides an interface to the X11 clipboard. Here, it is being used to copy its standard input into the clipboard.

```bash
# put this somewhere as a script in your PATH, shell alias, or keyboard shortcut
emocli -li | rofi -dmenu -window-title emoji | cut -d' ' -f1 | tr -d '\n' | xclip -selection clipboard
```

## Troubleshooting

### my terminal is not displaying all emoji characters

Your mileage may vary on terminal support for displaying emoji consisting of 2 or more characters joined by a zero-width joiner character (U+200D). The "service dog" emoji (🐕‍🦺) is one such example. Even with a proper font config, it will appear on many terminals as a dog next to a safety vest (🐕🦺).

Currently, the best configuration tested with `emocli -li` is the [kitty](https://github.com/kovidgoyal/kitty) terminal with the [Noto Color Emoji](https://github.com/DeeDeeG/noto-color-emoji-font) font installed and configured in your fontconfig.

## License

This software is released under the MIT License. See `LICENSE` for details.

This software includes source files from third party components, [Gitmoji](https://github.com/carloscuesta/gitmoji/) and [Unicode CLDR](https://cldr.unicode.org/). Each of these components have their own license. See `data/gitmoji/gitmoji-license.txt` and `data/emoji/unicode-license.txt`.

## Contributing

Pull requests are welcome at https://github.com/duhdugg/emocli/pulls

Submit issues at https://github.com/duhdugg/emocli/issues
