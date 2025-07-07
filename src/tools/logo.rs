use crate::_BOLD;
use crate::_RESET;

pub trait Logo {
    fn getlogo(&self) -> String;
}

pub struct Archlogo {
    logo: String,
}

impl Archlogo {
    pub fn new(color: &str) -> Self {
        let arch_logo = format!(
" {color}{_BOLD}                  -`                    {_RESET}
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

pub struct Ubuntulogo {
    logo: String,
}

impl Ubuntulogo {
    pub fn new(color: &str) -> Self {
        let ubuntu_logo = format!(
" {color}{_BOLD}                            ....          {_RESET}
 {color}{_BOLD}             .',:clooo:  .:looooo:.       {_RESET}
 {color}{_BOLD}          .;looooooooc  .oooooooooo'      {_RESET}
 {color}{_BOLD}       .;looooool:,''.  :ooooooooooc      {_RESET}
 {color}{_BOLD}      ;looool;.         'oooooooooo,      {_RESET}
 {color}{_BOLD}     ;clool'             .cooooooc.  ,,   {_RESET}
 {color}{_BOLD}        ...                ......  .:oo,  {_RESET}
 {color}{_BOLD} .;clol:,.                        .loooo' {_RESET}
 {color}{_BOLD}:ooooooooo,                        'ooool {_RESET}
'{color}{_BOLD}ooooooooooo.                        loooo.{_RESET}
'{color}{_BOLD}ooooooooool                         coooo.{_RESET}
 {color}{_BOLD},loooooooc.                        .loooo.{_RESET}
 {color}{_BOLD}  .,;;;'.                          ;ooooc {_RESET}
 {color}{_BOLD}      ...                         ,ooool. {_RESET}
 {color}{_BOLD}   .cooooc.              ..',,'.  .cooo.  {_RESET}
 {color}{_BOLD}     ;ooooo:.           ;oooooooc.  :l.   {_RESET}
 {color}{_BOLD}      .coooooc,..      coooooooooo.       {_RESET}
 {color}{_BOLD}        .:ooooooolc:. .ooooooooooo'       {_RESET}
 {color}{_BOLD}          .':loooooo;  ,oooooooooc        {_RESET}
 {color}{_BOLD}              ..';::c'  .;loooo:'         {_RESET}
 {color}{_BOLD}                                          {_RESET}
 {color}{_BOLD}                                          {_RESET}"
        );

        Self { logo: ubuntu_logo }
    }
}

impl Logo for Ubuntulogo {
    fn getlogo(&self) -> String {
        self.logo.clone()
    }
}


pub struct Debianlogo {
    logo: String,
}

impl Debianlogo {
    pub fn new(color: &str) -> Self {
        let ubuntu_logo = format!(
" {color}{_BOLD}                                 {_RESET}
 {color}{_BOLD}         , $$$$$$$$$$$$$          {_RESET}
 {color}{_BOLD}       ,g$$$$$$$$$$$$$$$$$$$      {_RESET}
 {color}{_BOLD}     ,g$$$$$            $$$$$$    {_RESET}
 {color}{_BOLD}    ,$$$$$$               $$$$$.  {_RESET}
 {color}{_BOLD}   ',$$$$$       ,ggs.     `$$$$: {_RESET}
 {color}{_BOLD}   `d$$$$'     ,$P$'   $    $$$$  {_RESET}
 {color}{_BOLD}    $$$$P      d$'     $    $$$$$ {_RESET}
 {color}{_BOLD}    $$$$:      $$.   $    ,d$$$$' {_RESET}
 {color}{_BOLD}    $$$$;      Y$b._   _,d$$$'    {_RESET}
 {color}{_BOLD}    Y$$$$.     `.$`$Y$$$$$4       {_RESET}
 {color}{_BOLD}    `$$$$b      -.__$$$$$         {_RESET}
 {color}{_BOLD}     $`Y$$                        {_RESET}
 {color}{_BOLD}      `Y$$$.                      {_RESET}
 {color}{_BOLD}        `$$$$.                    {_RESET}
 {color}{_BOLD}          `Y$$$$.                 {_RESET}
 {color}{_BOLD}             `Y$$._               {_RESET}
 {color}{_BOLD}                `$$$              {_RESET}
 {color}{_BOLD}                                  {_RESET}"
        );

        Self { logo: ubuntu_logo }
    }
}

impl Logo for Debianlogo {
    fn getlogo(&self) -> String {
        self.logo.clone()
    }
}

pub struct Fedoralogo {
    logo: String,
}

impl Fedoralogo {
    pub fn new(color: &str) -> Self {
        let fedora_logo = format!(
"{color}{_BOLD}                                        {_RESET}
{color}{_BOLD}                                         {_RESET}
{color}{_BOLD}               .'ccccc:.                 {_RESET}
{color}{_BOLD}          ,'ccccccccccccccc:.            {_RESET}
{color}{_BOLD}       .';cccccccccccccccccccc:.         {_RESET}
{color}{_BOLD}    .'ccccccccccccco@@@@bccccccc;        {_RESET}
{color}{_BOLD}   .:cccccccccccccd@@cccq@@:cccccc,      {_RESET}
{color}{_BOLD}   .:ccccccccccccc@@@ccccc@@Pccccccc;    {_RESET}
{color}{_BOLD} ,:ccccccccccccccc@@@ccccccccccccccc:    {_RESET}
{color}{_BOLD} ,:ccccccccccc@@@@@@c@@@@@@@ccccccccc:   {_RESET}
{color}{_BOLD} ;ccccccccd@ccccccc@@@ccccccccccccccc:   {_RESET}
{color}{_BOLD}  'ccccccc@@occccccc@@@ccccccccccccccc'  {_RESET}
{color}{_BOLD}  ;ccccccq@@bcccccd@@Pccccccccccccc;     {_RESET}
{color}{_BOLD}  ':ccccccccq@@@@@@@Pccccccccccccc:,     {_RESET}
{color}{_BOLD}   'cccccccccccccccccccccccccccc:.       {_RESET}
{color}{_BOLD}     ;ccccccccccccccccccccccc:,.         {_RESET}
{color}{_BOLD}        ,'ccccccccccccccccc:.,           {_RESET}
{color}{_BOLD}            .'ccccccccc:.                {_RESET}
{color}{_BOLD}                                         {_RESET}"
        );


        Self { logo: fedora_logo }
    }
}

impl Logo for Fedoralogo {
    fn getlogo(&self) -> String {
        self.logo.clone()
    }
}

pub struct Otherlogo {
    logo: String,
}

impl Otherlogo {
    pub fn new(color: &str) -> Self {
        let ubuntu_logo = format!(
" {color}{_BOLD}                                 {_RESET}
 {color}{_BOLD}         #####                    {_RESET}
 {color}{_BOLD}        #######                   {_RESET}
 {color}{_BOLD}        ##O#O##                   {_RESET}
 {color}{_BOLD}        #VVVVV#                   {_RESET}
 {color}{_BOLD}      ##  VVV  ##                 {_RESET}
 {color}{_BOLD}     #          ##                {_RESET}
 {color}{_BOLD}    #            ##               {_RESET}
 {color}{_BOLD}    #            ###              {_RESET}
 {color}{_BOLD}   QQ#           ##Q              {_RESET}
 {color}{_BOLD} QQQQQQ#       #QQQQQQ            {_RESET}
 {color}{_BOLD} QQQQQQQ#     #QQQQQQQ            {_RESET}
 {color}{_BOLD}   QQQQQ#######QQQQQ              {_RESET}
 {color}{_BOLD}                                  {_RESET}"
        );

        Self { logo: ubuntu_logo }
    }
}

impl Logo for Otherlogo {
    fn getlogo(&self) -> String {
        self.logo.clone()
    }
}
