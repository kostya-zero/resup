# Resup
<img src="imgs/logo.svg" width="128px" align="right">

Resup is CLI front-end for `realesrgan-ncnn-vulkan` image upscaler written in Rust.
It allows you quickly upscale your images directly in CLI.

## Installation

### Install Real-ESRGAN
Resup requires `realesrgan-ncnn-vulkan` to be installed. 
Check your distro repositories for their availability.
If its not available in your distro repositories, download it from [official repository](https://github.com/xinntao/Real-ESRGAN/releases).

### Install Resup

#### Install with cargo
Use command below to install it from crates.io (make sure that you have installed Rust toolchain and C/C++ compiler).
```shell
cargo install resup
```

#### Install from releases
Go to release tab and download version that you want. Extract archive content and place executable in directory that exists in `PATH` variable.

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
You can configure Resup for your system without need to edit configuration file. You can use Resup builtin commands for it.
- `model` - Shows currently used model. To specify new, use this command and as argument provide name of model. Example: `resup model realesrgan-x4plus-anime`. 
- `models-dir`  - Shows current path to directory with models. To specify new, use this command and as argument provide path to directory with models. Example: `resup models-dir ~/esrgan/models`.
- `executable`  - Shows current path to executable file. To specify new, use this command and as argument provide path to executable. Example: `resup executable ~/esrgan/realesrgan-ncnn-vulkan`.
