# Takenote

<image style="height:100px;width:100px;" src="assets/takenote-logos.jpeg" alt="Takenote Logo"/>

A secure, fast and extensible journal developed in Rust.

## Impetus

If you're an individual who actively part takes in note taking, you know how difficult it is to ensure the notes you write are
legible in the future. Maybe you write the notes in the morning, or quickly during a meeting or while you're contemplating if you
should sleep or watch another episode of Peaky Blinders. Whatever the case may be, you dislike effort but you like writing down your thoughts.
All you want to do is stay organized, ensure your notes are legible and most importantly, continue being lazy.

Based off the [Zettelkasten note taking system](https://en.wikipedia.org/wiki/Zettelkasten), **Takenote** aims to write high quality
journals based off small or not so small messages. Simply type your thought using the takenote command and we'll do the boring
things for you. When you look back, you'll be suprised how good of a writer you've become without putting any additional effort.

## Table Of Contents

- [Impetus](#impetus)
- [Usage](#usage)

## Usage

The following commands can be used to interact with takenote.

### Commands

```bash
    takenote init --name <project name> --children <path1> <path2> --path <path>
    takenote "ctrl-i jumps forward; ctrl-j to jump back in nvim."
    # Opens editor.
    takenote
    # Returns file names.
    takenote "find notes between 2022-01-01 and 2022-01-02"
```
