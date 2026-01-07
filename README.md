# Image Steganography Tool

A command-line steganography tool that hides text files inside images using LSB (Least Significant Bit) encoding.

## Features

- Hide text messages inside existing images
- Invisible to the human eye
- Extract hidden messages from images
- Works with common image formats (PNG, JPG)

## Installation

```bash
cargo build --release
```

## Usage

### Encode (Hide Message)

Hide a text file inside an image:

```bash
cargo run encode <text_file> <input_image> <output_image>
```

**Example:**
```bash
cargo run encode secret.txt cat.jpg output.png
```

### Decode (Extract Message)

Extract hidden message from an image:

```bash
cargo run decode <stego_image> <output_text>
```

**Example:**
```bash
cargo run decode output.png decoded.txt
```

## How It Works

The tool uses LSB steganography to hide data by modifying the least significant bit of each RGB color channel. This creates imperceptible changes to the image while storing your secret message.

## Limitations

- Message size limited by image dimensions (3 bits per pixel)
- Lossy compression (like JPEG re-encoding) will destroy hidden data
- Save encoded images as PNG to preserve data