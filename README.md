# lib.rs
<p align="center">
  <img src="https://github.com/cmleinz/lib.rs/blob/main/demo/book.png?raw=true" />
  <br>
  <img src="https://img.shields.io/badge/VERSION-0.0.1-blue" />
  <img src="https://img.shields.io/badge/LICENSE-MIT-blueviolet" />
</p>

A TUI frontend for browsing [arxiv](https://arxiv.org). 

## Disclaimers 

DISCLAIMER: THIS PROJECT IS IN NO WAY AFFILIATED WITH ARXIV.ORG, IT SIMPLY USES THE PUBLIC API TO GATHER RESULTS.

Additionally, this project is in very early stages of development. While it functions, I have not carefully cleaned the code, nor have I done proper error handling. These will be conducted later.

## Installation
For now the program is only installable with rust and cargo. Simply git clone the repo:

```bash
git clone https://github.com/cmleinz/lib.rs.git && cd lib.rs
```

And run the program with:

```bash
cargo r
```

## Introduction

![search.png](https://github.com/cmleinz/lib.rs/blob/main/demo/search.png?raw=true)

To begin a search type either 'i' or 's' and begin typing your search, then simply hit enter to query arxiv.org. The results will then be displayed as a list, as the user navigates the list, the right-side panel will be populated with details of the particular article. To open the highlighted paper, simply press enter.

The program in general uses vim-esque keybindings, with "Normal Mode" being associated with selecting and viewing articles, and insert mode being associated with searching arxiv.

The current mode is indicated in the modeline and in the color of the border of the various elements of the TUI. Red indicates that the region is active, while white indicates it is inactive.

## Attributions
[Book icons created by Smashicons - Flaticon](https://www.flaticon.com/free-icons/book) 

