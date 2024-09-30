# ccfetch
A neofetch like system information tool written in rust. Which is super fast, even faster than [fastfetch](https://github.com/fastfetch-cli/fastfetch) 

![](https://raw.githubusercontent.com/charleschetty/ccfetch/dev/shots/2024-09-30_16-46.png)

Currently, ccfetch only support Arch, Debian, and Ubuntu, other Linux distribution/OS may work, but 
the logo will be displayed as ["TUX"](https://en.wikipedia.org/wiki/Tux_(mascot)). 

# Installation

Install [Rust](https://www.rust-lang.org/tools/install) if you don't have. 

```shell
cargo build --release
# copy ccfetch from target/release to desired path you want
```

*note : In order to display icons correctly in the  terminal, you should install [nerd font](https://github.com/ryanoasis/nerd-fonts) first.*

# Todo

- [ ] More logos
- [ ] User configuration 



# Benchmark

![](https://raw.githubusercontent.com/charleschetty/ccfetch/dev/shots/2024-09-30_17-09.png)

I use AMD 4800u ,and the [fastfetch](https://github.com/fastfetch-cli/fastfetch) was compiled with :

```cmake
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_compile_options(
"$<$<COMPILE_LANGUAGE:CXX>:-O3;-march=native;-fpic;-ftree-vectorize>"
"$<$<COMPILE_LANGUAGE:C>:-O3;-march=native;-fpic;-ftree-vectorize>"
)
```

and the jsonc file I use is in <code>benchmark/ </code>, I configure it from the <code>fastfetch/presets/examples/2.jsonc</code>

# Reference 

This project inspired by [pfetch-rs](https://github.com/Gobidev/pfetch-rs), [treefetch](https://github.com/angelofallars/treefetch),  [macchina](https://github.com/Macchina-CLI/macchina) ,[fastfetch](https://github.com/fastfetch-cli/fastfetch)
