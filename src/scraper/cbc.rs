use select::document::Document;
use select::predicate::{Class, Name};
use crate::news::{NewsEnum, NewsSite, News};
use crate::scraper;

/// # scrape_cbc
/// This function is used to scrape cbcs home page
/// in the more stories area. It then returns a 
/// Vec<News> which contains information on the articles
/// which have been scraped.
/// 
#[allow(dead_code)]
pub async fn scrape_cbc(doc: Document, cbc: &NewsEnum) -> Vec<News> {
    //this is the first part of the page with non tabloid articles
    let more_stories_list = doc.find(Class("contentListItem"));
    let mut news_vec: Vec<News> = Vec::new();
    for node in more_stories_list {
        let link = node
            .find(Name("a"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap_or(""); 
        let check = check_link(link);
        //if the link does not pass the check then it will not scrape data from it.
        if check {
            let img_link = node
                .find(Name("img"))
                .next()
                .unwrap()
                .attr("src")
                .unwrap_or("");
            let title = node
                .find(Name("h3"))
                .next()
                .unwrap()
                .text();
            let metadata = node
                .find(Class("metadataText"))
                .next()
                .unwrap()
                .text();

            let site = NewsSite::get_site(&cbc);

            let article_url = site + link;
            
            let news = News::new(
                cbc.clone(), 
                article_url, 
                String::from(img_link),
                title,
                String::from(""), 
                String::from(""), 
                String::from(""), 
                metadata);

            news_vec.push(news); 
        }
    }
    news_vec
}
/// ## scrape_article
/// this function scrapes an article on cbc from the provided
/// document. It returns a tuple with the missing fields from the 
/// initial scrape. scrape_articles then puts them into the
pub async fn scrape_article(doc: Document) -> (String, String, String, String) {
    //scrape the authors name if there is one
    let details = doc
        .find(Class("bylineDetails"))
        .next();
    let auth;
    if let Some(val) = details {
        let author = val
                    .find(Name("span"))
                    .next();
        if let Some(val) = author {
            if let Some(val) = val.first_child() {
                auth = val.text();
            } else {
                auth = String::from("No - Author");
            }
        } else {
            auth = String::from("No - Author");
        }      
    } else {
        auth = String::from("No - Author");
    }
    //scrape time info on the page a string.
    let time = doc
        .find(Class("bylineDetails"))
        .next();
    let tme;
    if let Some(val) = time {
        let t = val.find(Name("time"))
                   .next();
        if let Some(val) = t {
            tme = val.text();
        } else {
            tme = String::from("No - Time Data");
        }
    } else {
        tme = String::from("No - Time Data");
    }
    //scrape the description of the article
    let desc = doc
        .find(Name("h2"))
        .next();
    let dsc;
    if let Some(val) = desc {
        let d = val;
        dsc = d.text();
    } else {
        dsc = String::from("No - Description");
    }
    //scrape the article text from the url.
    let text = doc
        .find(Class("story"))
        .next();
    let txt;
    if let Some(val) = text {
        let t = val;
        txt = t.text();
    } else {
        txt = String::from("No - Text");
    }
    
    ( auth, tme, dsc, txt)
}
/// ## scrape_articles
/// Cbcs implementation of scrape_articles
/// this function uses the scrape_article function to grab
/// all the articles in the provided News Vector.
pub async fn scrape_articles(news: Vec<News>) -> Vec<News>
{
    let mut news_vec: Vec<News> = Vec::new();
    for mut new in news {              
        let doc = scraper::get_document(&new.article_link).await;
        let missing_data = scrape_article(doc).await;
        new.add_scrape_article(missing_data);
        news_vec.push(new);
    }
    news_vec
}

fn check_link(check_string: &str) -> bool{
    let split: Vec<&str> = check_string.split('/').collect();
    //Check the 2nd value in the vector as the first will be blank.
    //check against news as that is the route on cbc which has actual 
    //articles which are worth scraping.
    if split[1] == "news" { true } else { false }
}