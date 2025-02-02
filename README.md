
<!-- README.md is generated from README.Rmd. Please edit that file -->

# urlparser

<!-- badges: start -->

<!-- badges: end -->

A simple (but fast) url parser that wraps the crate
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

## Benchmark

This short benchmakr shows that this package outperforms
[adaR](https://github.com/gesistsa/adaR), a highly optimized parser
written in C++.

``` r
top100 <- readLines(
  "https://raw.githubusercontent.com/ada-url/url-various-datasets/main/top100/top100.txt"
)

bench::mark(
  check = FALSE,
  adaR::ada_url_parse(top100),
  urlparser::rs_url_parse(top100)
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
#> # A tibble: 2 × 6
#>   expression                           min   median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr>                      <bch:tm> <bch:tm>     <dbl> <bch:byt>    <dbl>
#> 1 adaR::ada_url_parse(top100)        279ms    293ms      3.41    29.6MB     5.11
#> 2 urlparser::rs_url_parse(top100)    123ms    123ms      8.09    8.28MB     0
```
