#' Use url crate to parse a url
#' @param url character. one or more URL to be parsed
#' @return A data frame of the url components:
#' href, protocol, username, password, host, hostname, port, pathname, search, and hash
#' @examples
#' rs_url_parse("https://user_1:password_1@example.org:8080/dir/../api?q=1#frag")
#' @export

rs_url_parse <- function(url){
  url_parse(url)
}