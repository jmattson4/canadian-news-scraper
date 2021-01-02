use reqwest;
use tokio;
use select::document::Document;

use crate::news::{NewsEnum, NewsSite, News};

/// # cbc
/// This module holds scraping logic related to cbc_news.
mod cbc;
/// # ctv
/// This module holds scraping logic related to ctv_news.
mod ctv;
/// # global_news
/// This module holds scraping logic related to global_news.
mod global_news;

/// ## scrape
/// This function takes a NewsEnum and will then scrape
/// the site. There is different logic that runs related
/// to each website and the logic is stored inside the modules
/// named after the websites.
#[tokio::main]
pub async fn scrape(news: NewsEnum) -> Vec<News> {
    let site = NewsSite::get_site(&news);

    let doc = get_document(&site).await;

    let news_scrape = choose_initial_scrape(&news, doc).await;
    let news_scrape = scrape_articles(&news, news_scrape).await;


    news_scrape
}
// pub async fn scrape_all() -> (Vec<News>, Vec<News>, Vec<>)
/// ## get_document
/// 
/// This function is used to return an html Document from
/// a url string. The document is used to scrape information
/// from whatever website the url points too
/// 
/// Need to make this more error friendly without panics
pub async fn get_document(url: &String) -> select::document::Document {
    let body = reqwest::get(url).await;
    let res = match body {
        Ok(res) => res,
        Err(err) => panic!("Error: {}", err)
    };
    assert!(res.status().is_success());

    let res_text = res.text().await;
    let res_text = match res_text {
        Ok(text) => text,
        Err(err) => panic!("Could not read document from response text, Err: {}", err)
    };

    let document = Document::from_read(res_text.as_bytes());
    let read_result = match document {
        Ok(doc) => doc,
        Err(err) => panic!("Could not read document from response text, Err: {}", err)
    };

    read_result
}


/// ## choose_initial_scrape
/// This function gets the initial scrape depending on the news site.
/// 
/// It returns a Vector with news structures inside which can be used
/// to further scrape the articles.
#[allow(dead_code)]
async fn choose_initial_scrape<'a>(news: &NewsEnum, doc: Document) -> Vec<News>{
    match news {
        NewsEnum::CBC => cbc::scrape_cbc(doc, news).await,
        NewsEnum::CTV => ctv::scrape_ctv(doc, news).await,
        NewsEnum::GlobalNews => global_news::scrape_global_news(doc, news).await,
    }
}
/// ## scrape_articles
/// This function is used to scrape the article_links which have been grabbed
/// in the initial scrape.
#[allow(dead_code)]
async fn scrape_articles(news_site: &NewsEnum, news: Vec<News>) -> Vec<News> {
    match news_site {
        NewsEnum::CBC => {
            cbc::scrape_articles(news).await
        },
        NewsEnum::CTV =>  {
            ctv::scrape_articles(news).await
        },
        NewsEnum::GlobalNews => {
            global_news::scrape_articles(news).await
        },
    }
}