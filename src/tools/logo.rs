use crate::_GREEN;
use crate::_RESET;
use crate::_BOLD;

pub trait Logo {
    fn getlogo(&self) -> String;
}

pub struct Archlogo {
    logo: String,
}

impl Archlogo {
    pub fn new(color : &str) -> Self {
        let arch_logo = format!(
" {_GREEN}{_BOLD}                  -`                    {_RESET}
 {color}{_BOLD}                 .o+`                   {_RESET}
 {color}{_BOLD}                `ooo/                   {_RESET}
 {color}{_BOLD}               `+oooo:                 {_RESET} 
 {color}{_BOLD}              `+oooooo:                 {_RESET}
 {color}{_BOLD}              -+oooooo+:                {_RESET}
 {color}{_BOLD}            `/:-:++oooo+:               {_RESET}
 {color}{_BOLD}           `/++++/+++++++:              {_RESET}
 {color}{_BOLD}          `/++++++++++++++:             {_RESET}
 {color}{_BOLD}         `/+++ooooooooooooo/`           {_RESET}
 {color}{_BOLD}        ./ooosssso++osssssso+`          {_RESET}
 {color}{_BOLD}       .oossssso-````/ossssss+`         {_RESET}
 {color}{_BOLD}      -osssssso.      :ssssssso.        {_RESET}
 {color}{_BOLD}     :osssssss/        osssso+++.       {_RESET}
 {color}{_BOLD}    /ossssssss/        +ssssooo/-       {_RESET}
 {color}{_BOLD}  `/ossssso+/:-        -:/+osssso+-     {_RESET}
 {color}{_BOLD} `+sso+:-`                 `.-/+oso:    {_RESET}
 {color}{_BOLD}`++:.                           `-/+/   {_RESET}
 {color}{_BOLD}.`                                 `/   {_RESET}
 {color}{_BOLD}                                        {_RESET}"
        );

        Self { logo: arch_logo }
    }
}

impl Logo for Archlogo {
    fn getlogo(&self) -> String {
        self.logo.clone()
    }
}
