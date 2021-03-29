# Hex Viewer

A hex viewer for terminal written in rust. This project is in very early stages of development. Feel free to send me any ideas or PR. ;)

![image](demo.gif)

## Installation

```bash
git clone https://github.com/vvayn3tseng/hex_viewer.git
cd hex_viewer
cargo run
```

## Usage

Use tab to switch between blocks. 

## Currently available command

- open \<file_name>
- jump \<offset>
- quit

## Future plan

- Viewer highlight
- Blocks shows ascii
- Open multiple files
- Help command

## Should fix in the future

- LRU for buffer cache
- Handle terminal resize event better
- Set cursor more precisely