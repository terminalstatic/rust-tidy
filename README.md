# rust-tidy

A wrapper library for [Html Tidy](https://github.com/htacg/tidy-html5) in rust.

## Requirements

Requires common build tools and libtidy (>=5.2.0), 
for example on ubuntu install with

````sudo apt install libtidy-dev````

on mac with

````brew install tidy-html5````


The implementation provides access to a major but not complete part of html tidys api in a (hopefully) more rust friendly way.<br />
Please be aware that this lib is a pet project and therefore not thoroughly tested.<br />
Current api docs can be browsed [here](https://terminalstatic.github.io/rust-tidy/tidy/index.html).
