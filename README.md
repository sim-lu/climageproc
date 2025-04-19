# climageproc

A command-line tool for batch image processing written in Rust. Supports resizing images and converting between formats.

## Features

- Resize images while maintaining aspect ratio
- Convert images between different formats (JPG, PNG, GIF, WebP)
- Batch processing support for directories
- Progress bar for tracking operations
- Parallel processing for better performance

## Installation

1. Make sure you have Rust installed on your system
2. Clone this repository
3. Build the project:
```bash
cargo build --release
```

## Usage

### Resize Images

Resize a single image:
```bash
climageproc resize -i input.jpg -o output.jpg -w 800 -h 600
```

Resize all images in a directory:
```bash
climageproc resize -i input_dir -o output_dir -w 800
```

Options:
- `-i, --input`: Input file or directory
- `-o, --output`: Output file or directory
- `-w, --width`: New width in pixels
- `-h, --height`: New height in pixels

### Convert Format

Convert a single image:
```bash
climageproc convert -i input.jpg -o output_dir -f png
```

Convert all images in a directory:
```bash
climageproc convert -i input_dir -o output_dir -f webp
```

Options:
- `-i, --input`: Input file or directory
- `-o, --output`: Output file or directory
- `-f, --format`: Target format (jpg, png, gif, webp)

## Supported Formats

- JPEG/JPG
- PNG
- GIF
- WebP