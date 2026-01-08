# Image Steganography Tool

A command-line steganography tool that hides text files inside images using LSB (Least Significant Bit) encoding.

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
