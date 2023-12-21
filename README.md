# ccfetch
A neofetch like system information tool written in rust.

![](https://raw.githubusercontent.com/charleschetty/ccfetch/main/shots/2023-11-26_06-31.png)

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


# Reference 
This project inspired by [pfetch-rs](https://github.com/Gobidev/pfetch-rs), [treefetch](https://github.com/angelofallars/treefetch), and [macchina](https://github.com/Macchina-CLI/macchina) 
