# gmocli

gmocli provides a command-line interface for searching emoji characters with
associated gitmoji descriptions.

It uses combined data from [Mange/emoji-data](https://github.com/Mange/emoji-data) and [carloscuesta/gitmoji](https://github.com/carloscuesta/gitmoji).

**Homepage:** https://github.com/duhdugg/gmocli

## Usage

```text
Usage:	gmocli [OPTIONS] <search>

OPTIONS:
-h | --help 	print this help
-l | --list 	list all emoji characters
-i | --info 	include info
-n | --name 	match name exactly

  --version 	print version and exit
```

### Examples

#### list all emoji characters with their name, info, and gitmoji description (if available)

`gmocli -li`

#### search for an emoji using keywords

```bash
gmocli -i dog
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
gmocli -n 'guide dog'
# output
🦮
```

#### use in a commit message

`git commit -m "$(gmocli -n rocket) production launch"`

#### use with dmenu, fzf, or rofi

For interactive searching, you can pipe the output of `gmocli -li` to something
like dmenu, fzf, or rofi as follows:

```bash
gmocli -li | fzf | cut -d' ' -f1 | tr -d '\n'  # | xclip or whatever
```

## Troubleshooting

### my terminal is not displaying all emoji characters

Your mileage may vary on terminal support for displaying emoji consisting of 2 or more characters joined by a zero-width joiner character (U+200D). The "service dog" emoji (🐕‍🦺) is one such example. Even with a proper font config, it will appear on many terminals as a dog next to a safety vest (🐕🦺).

Currently, the best configuration tested with `gmocli -li` is the [wezterm](https://github.com/wez/wezterm) terminal with the [Noto Color Emoji](https://github.com/DeeDeeG/noto-color-emoji-font) font installed and configured in your fontconfig. [kitty](https://github.com/kovidgoyal/kitty) also works well.

## License

This software is released under the MIT License. See `LICENSE` for details.

This software includes source files from third party components, [Gitmoji](https://github.com/carloscuesta/gitmoji/) and [Unicode CLDR](https://cldr.unicode.org/). Each of these components have their own license. See `data/gitmoji/gitmoji-license.txt` and `data/emoji/unicode-license.txt`.

## Contributing

Pull requests are welcome at https://github.com/duhdugg/gmocli/pulls

Submit issues at https://github.com/duhdugg/gmocli/issues
