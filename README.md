# ccfetch

A neofetch like system information tool written in rust. Which is super fast, even faster than [fastfetch](https://github.com/fastfetch-cli/fastfetch)

![screenshot](/screenshots/screenshot1.png)

Currently, ccfetch only supports Arch, Debian, Fedora, and Ubuntu, but the logo will be displayed as ["TUX"](https://en.wikipedia.org/wiki/Tux_(mascot)) if you use other distributions

## Installation

Download from [actions](https://github.com/charleschetty/ccfetch/actions/workflows/build.yml)

### Build from source

> You should install [rust](https://www.rust-lang.org/tools/install) first

```shell
git clone https://github.com/charleschetty/ccfetch
cd ccfetch
cargo build --release
# then copy ccfetch under target/release/ to desired path you want
```

*note : In order to display icons correctly in the terminal, you should install [nerd font](https://github.com/ryanoasis/nerd-fonts) first.*

## Todo

- [ ] More distribution logos
- [ ] User configuration

## Benchmark

![benchmark](/screenshots/benchmark.png)

I use AMD 4800u ,and the [fastfetch](https://github.com/fastfetch-cli/fastfetch) was compiled with :

```cmake
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_compile_options(
"$<$<COMPILE_LANGUAGE:CXX>:-O3;-march=native;-fpic;-ftree-vectorize>"
"$<$<COMPILE_LANGUAGE:C>:-O3;-march=native;-fpic;-ftree-vectorize>"
)
```

`$ cmake .. -GNinja -L | grep -oP 'PACKAGES_DISABLE_\w+' | grep -v PACMAN | sed -e 's/.*/-D\0=ON/' | xargs cmake .. -GNinja -DCMAKE_BUILD_TYPE=Release -DENABLE_EMBEDDED_PCIIDS=ON`

and the jsonc file I use is in `benchmark/`, I configure it from the `fastfetch/presets/examples/2.jsonc`

## Reference

This project inspired by [pfetch-rs](https://github.com/Gobidev/pfetch-rs), [treefetch](https://github.com/angelofallars/treefetch),  [macchina](https://github.com/Macchina-CLI/macchina) ,[fastfetch](https://github.com/fastfetch-cli/fastfetch)
