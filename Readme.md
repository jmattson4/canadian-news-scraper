# Canadian News Scraper
###  by Jace Mattson

This is a Rust library which can be included in other programs to pull data
from at the moment three canadian news sites.

- Global News 
    - https://www.cbc.ca/
    - scrapes top 4.
- Ctv News
    - https://www.ctvnews.ca/
    - scrapes top 12.
- Cbc News
    - https://globalnews.ca/
    - scrapes top 12.

## Explanation

The library api is fairly simple it works by providing the 
scrape() function - which is located in the scraper module - which takes a NewsEnum that it then uses to scrape the site associated
to that Enum. 

```
pub async fn scrape(news: NewsEnum) -> Vec<News>
```
scrape() uses a series of functions located in the scraper module to determine which News Site markup it needs to use to effectively scrape the website you've requested.


## Data Structure
The underlying data structure that scrape() returns is a Vector of type News. News is a struct which is defined in the 
News module alongside NewsEnum and NewsSite.

``` rs
pub struct News {
    pub news_enum: NewsEnum,
    pub news_site: String,
    pub article_link: String,
    pub img_link: String,
    pub title: String,
    pub desc: String,
    pub author: String,
    pub metadata: String,
    pub article_text: String,
    pub article_date: String,
    pub scrape_date: DateTime<Utc>
}
```
## Use Case
The main use case of this program is in a REST API I'm creating which will be used to scrape these sites on a daily
basis and store the information in a database which can then be accessed via the REST application.

## Example 

``` rs

let news_site = news::NewsEnum::GlobalNews;
let val:Vec<News> = scraper::scrape(news_site);


```

You can see similiar examples in the tests module located in lib.rs.