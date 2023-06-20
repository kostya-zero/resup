# Resup
Resup is CLI front-end for `realesrgan-ncnn-vulkan` image upscaler written in Rust.
It allows you quickly upscale your images directly in CLI.

## Installation

Resup requires `realesrgan-ncnn-vulkan` to be installed. 
Check your distro repositories for their availability.
If its not available in your distro repositories, download it from [official repository](https://github.com/xinntao/Real-ESRGAN/releases).

You can install Resup by downloading from releases page or by compiling it. 
For second one, read [Build section](#build).

## Usage

Use `upscale` subcommand to tell Resup upscale specific image.

```shell
resup upscale -i "INPUT_FILE" -o "OUTPUT_FILE" -m "photo"
```

### Arguments legend

- `-i`, `--input` - Input file name. This file will be used in upscaling.
- `-o`, `--output` - Output file name. Final image file.
- `-m`, `--model` - Model to use. Can be `photo` if you want to upscale photo or `anime` for anime pictures.

## Build 

To increase performance of Resup we recommend you to use `RUSTFLAGS` such as `opt-level` and `target-cpu`. 
Read more about `RUSTFLAGS` on the [Cargo Book Page](https://doc.rust-lang.org/cargo/reference/environment-variables.html).

```shell
env RUSTFLAGS="-C target-cpu=native -C opt-level=2" cargo build --release
```

> WARNING: Optimization level 3 is not recommended because on some certain systems it might decrease performance of Resup.
