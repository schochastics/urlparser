
<!-- README.md is generated from README.Rmd. Please edit that file -->

# urlparser

<!-- badges: start -->
<!-- badges: end -->

A simple url parser that wraps the crate
[url](https://crates.io/crates/url).

## Installation

You can install the development version of urlparser from
[GitHub](https://github.com/) with:

``` r
# install.packages("pak")
pak::pak("schochastics/urlparser")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(urlparser)
rs_url_parse("https://user_1:password_1@example.org:8080/dir/../api?q=1#frag")
#>                                                              url scheme
#> 1 https://user_1:password_1@example.org:8080/dir/../api?q=1#frag  https
#>          host port path query fragment username   password
#> 1 example.org 8080 /api   q=1     frag   user_1 password_1
```
