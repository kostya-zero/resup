# Resup


Resup is CLI front-end for `realesrgan-ncnn-vulkan` image upscaler written in Rust.
It allows you quickly upscale your images directly in CLI.

## Requirements

Resup requires `realesrgan-ncnn-vulkan` to be installed.
Check your distro repositories for their availability.
If its not available in your distro repositories, download it from [official repository](https://github.com/xinntao/Real-ESRGAN/releases).

## Installation

You can install Resup from crates.io with cargo (make sure that you have installed Rust toolchain and C/C++ compiler):

```shell
cargo install resup
```

Also, you can use precompiled binaries from releases.
Go to releasse tab and download version that you want. Extract archive content and place executable in directory that exists in `PATH` variable.


#### Build from source
You can compile Resup manually. Clone this repository, install Rust toolchain and C/C++ compiler and run build with command below.
```shell
cargo build --release
```

## Usage
To start upscale you can use `upscale` command with the path to your file:
```shell
resup upscale my_image.jpg
```

In this example, image will be saved as `my_image-upscaled.png`. You can specify name of output image manually with `--output` argument.
```shell
resup upscale my_image.jpg --output my_image.png
```

You can get full list of available models. Use `list` command for it:
```shell
resup list
```

## Configuration

TBD
