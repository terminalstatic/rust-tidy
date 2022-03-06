# rust-tidy

A wrapper library for [HTML Tidy](https://github.com/htacg/tidy-html5) in rust.

## Overview

[HTML Tidy](https://github.com/htacg/tidy-html5) corrects and cleans up HTML and XML documents by fixing markup errors and upgrading legacy code to modern standards.<br />
This wrapper provides access to a major subset of the [HTML Tidy](https://github.com/htacg/tidy-html5) api.<br />
Please be aware that this lib is a pet project and therefore not thoroughly tested.<br />

## Requirements

Requires the usual build tools and libtidy (>=5.2.0), 
for example on ubuntu install with

````sudo apt install libtidy-dev````

on mac with

````brew install tidy-html5````

## Usage

Most of the time it should be possible to add the library to your dependencies like this: 
````
# Cargo.toml

[dependencies]
tidy = { git = "https://github.com/terminalstatic/rust-tidy", branch = "master" }
````
or pin it with
````
# Cargo.toml

[dependencies]
tidy = { git = "https://github.com/terminalstatic/rust-tidy", tag = "tidy-v0.1.7" }
````

However the build script might not work everywhere (wrote it for ubuntu and macOS). 

## Api Docs

Current **api docs** can be browsed [here](https://terminalstatic.github.io/rust-tidy/tidy/index.html).
