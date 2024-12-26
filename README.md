# Jot

A simple aid for note-taking, written in Rust.

It aims to provide a little added utility to help [Live the Plain Text Life](http://www.markwk.com/plain-text-life.html), complementing a Vim and fuzzy finder-based workflow.

Inspired by:

- [Noted](https://github.com/scottashipp/noted)
- [Note-Taking in Vanilla Vim](https://www.edwinwenink.xyz/posts/42-vim_notetaking/)

## Usage

Jot has three main commands, use `jot --help` or `jot [command] --help` for more information on usage:

- `jot` (without a subcommand): Used to generate new "atoms" - short, timestamped Markdown notes
- `jot html`: Convert any Markdown document to HTML and output to stdout or a file
- `jot pdf`: Convert any Markdown document to a PDF file using headless chrome

Note that PDF conversion requires chrome installed and available on PATH.
Other features will work as normal.

## Installation

Download one of the precompiled binaries in [releases](https://github.com/bratter/jot/releases).
Precompiled binaries are available for `x86-64-unkonwn-linux-gnu` and `x86_64-pc-windows-gnu` targets.
Install anywhere in PATH, recommend `~/.local/bin/`.

Jot can also be installed using cargo:

```bash
git clone https://github.com/bratter/jot.git
cd jot
cargo install jot
```

## Roadmap

The following fixes/features are on the agenda

- [x] Basic create and save note functionality
- [x] Add ability to output Markdown as HTML
- [x] Add ability to output Markdown as PDF
- [x] Output HTML/PDFs don't need a destination filename - add 'o' switch
- [ ] Improve error messages with anyhow contexts
- [ ] Improve Markdown parsing and rendering (likely to be implemented in a shared lib crate wrapping cmark_pulldown:
  - Look into the markdown parser ecosystem, including https://github.com/wooorm/markdown-rs
  - Support processing front matter in files - this will come after creating separate crate for rendering markdown to use with other notes-related apps also
  - Don't write Markdown to string then to a file (requires changing pulldown's writer
  - Pull title in from `<h1>` if it's not in the front matter
  - Pull modified date from file stamp if not in front matter
- [ ] Flag for opening when HTML/PDF are made
- [ ] Add ability to pull CSS for rendering from both default file in config and cli argument (currently pulls from root dir then config dir, but not ability to configure)
- [ ] Programmatically modify config, and improve relationship with env var overwriting
- [x] Make EDITOR env var not required to build the config
- [ ] Improve UX for managing the `notes` folder overall, not just the `atoms` subfolder
- [ ] Support custom front matter strings (that might need strfmt to work
- [ ] Fix: Stop headless chrome launching a window on Windows
- [ ] Fix: Formatting of path strings when printed on Windows
- [ ] Add some regression tests
- [ ] Create a neovim plugin that adds jot commands and other note management items
