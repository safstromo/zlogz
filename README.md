# zlogz

A simple tool for creating a daily markdown log.

## Install

Clone the repo:
`https://github.com/safstromo/zlogz.git`

Move to repo:
`cd zlogz`

Install:

`cargo install --path .`

## Usage

Run with:

`zlogz`

This creates a markdown file with todays date(Example 2024-06-06.md),
and opens it in the defined editor.

To search files(only work with nvim and telescope)

`zlogz -f`

To search all(only work with nvim and telescope)

`zlogz -s`

## Config

Create a .zlogz.toml in your ~/config

### Default

`path = "/home/username/zlogz"
editor = "nvim"`
