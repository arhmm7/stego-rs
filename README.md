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
encrypt encode <text_file> <input_image> <output_image>
```

**Example:**
```bash
encrypt encode secret.txt cat.jpg output.png
```

### Decode (Extract Message)

Extract hidden message from an image:

```bash
encrypt decode <stego_image> <output_text>
```

**Example:**
```bash
encrypt decode output.png decoded.txt
```

## How It Works

The tool uses LSB steganography to hide data by modifying the least significant bit of each RGB color channel. This creates imperceptible changes to the image while storing your secret message.
