# emocli

emocli provides a command-line interface for searching emoji characters.

It uses combined data from [muan/unicode-emoji-json](https://github.com/muan/unicode-emoji-json) and [carloscuesta/gitmoji](https://github.com/carloscuesta/gitmoji).

## Install

`npm install -g emocli`

or

`yarn global install emocli`

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

#### list all emoji characters with their name / gitmoji description (if available)

`emocli -li`

#### search for an emoji using keywords

```bash
emocli -i dog
# output
🐶 dog face
🐕 dog
🦮 guide dog
🐕‍🦺 service dog
🌭 hot dog
```

#### print a specific emoji by name

if the name is more than one word, it should be enclosed in quotes

```bash
emocli -n 'guide dog'
# output
🦮
```

#### use in a commit message

`git commit -m "$(emocli rocket) production launch"`

#### use with rofi and xclip

[rofi](https://github.com/davatorium/rofi) is a window switcher, application launcher, and dmenu replacement. Here, it is being used as a dmenu replacement to provide a graphical interface for search and select.

[xclip](https://github.com/astrand/xclip) is a command line utility that provides an interface to the X11 clipboard. Here, it is being used to copy its standard input into the clipboard.

```bash
# put this somewhere as a script in your PATH, shell alias, or keyboard shortcut
emocli -li | rofi -dmenu -window-title emoji | cut -d' ' -f1 | tr -d '\n' | xclip -selection clipboard
```

## Troubleshooting

### my terminal is not displaying all emoji characters

Your mileage may vary on terminal support for displaying emoji consisting of 2 characters joined by a zero-width joiner character (U+200D). The "service dog" emoji (🐕‍🦺) is one such example. Even with a proper font config, it will appear on many terminals as a dog next to a safety vest (🐕🦺).

Currently, the best configuration tested with `emocli -li` is the [kitty](https://github.com/kovidgoyal/kitty) terminal with the [Noto Color Emoji](https://github.com/DeeDeeG/noto-color-emoji-font) font installed and configured in your fontconfig.

## License

```
Copyright (c) 2021 Doug Elkin

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
