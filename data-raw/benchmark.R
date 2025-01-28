library(adaR)
top100 <- readLines("https://raw.githubusercontent.com/ada-url/url-various-datasets/main/top100/top100.txt")

bench::mark(check=FALSE,
  ada_url_parse("https://user_1:password_1@example.org:8080/dir/../api?q=1#frag"),
  rs_url_parse("https://user_1:password_1@example.org:8080/dir/../api?q=1#frag")
)
bench::mark(check=FALSE,
  ada_url_parse(top100),
  rs_url_parse(top100)
)