use select::document::Document;
use select::predicate::{Class, Name};
use crate::news::{NewsEnum, News};
use crate::scraper;

/// ## scrape_ctv
/// This function is the ctv implementation for the initial
/// scrape.
pub async fn scrape_ctv(doc: Document, ctv: &NewsEnum) -> Vec<News> {
    let mut news_vec: Vec<News> = Vec::new();
    let top_twelve = doc
        .find(Name("article"))
        .take(12);
    for node in top_twelve {
        //Get link from page
        let link = node.find(Name("a")).next();
        let link_text;
        if let Some(val) = link {
            link_text = val.attr("href").unwrap_or("No - Link");
        } else {
            link_text = "No - Link";
        }
        //get title from page
        let title = node.find(Name("div")).next();
        let title_text;
        if let Some(val) = title {
            let t = val.find(Name("h3")).next();
            if let Some(val) = t {
                title_text = String::from(val.text().trim());
            } else {
                title_text = String::from("No - Title")
            }
        } else {
            title_text = String::from("No - Title")
        }
        //get desc from page
        let desc = node.find(Class("c-list__item__description")).next();
        let desc_text;
        if let Some(val) = desc {
            desc_text = val.text();
        } else {
            desc_text = String::from("No - Description");
        }

        //get image link
        let img = node.find(Name("img")).next();
        let img_link;
        if let Some(val) = img {
            img_link = val.attr("src").unwrap_or("No - Image Link");
        } else {
            img_link = "No - Image Link";
        }   
        //create new news struct
        let new = News::new(
            ctv.clone(), String::from(link_text), 
            String::from(img_link), title_text, 
            desc_text, String::from(""),
            String::from(""), String::from(""));
        //add news struct to the news_vec
        news_vec.push(new);
    }


    news_vec
}
/// scrape_articles
/// 
/// this is the scrape_arcticles implementation for ctv
/// it calls the scrape_article function a number of times
/// in order to get all the remaining information from the article
/// page
/// 
pub async fn scrape_articles(news: Vec<News>) -> Vec<News> {
    let mut news_vec: Vec<News> = Vec::new();
    for mut new in news {              
        let doc = scraper::get_document(&new.article_link).await;
        let missing_data = scrape_article(doc).await;
        new.add_scrape_article(missing_data);
        news_vec.push(new);
    }
    news_vec
}

pub async fn scrape_article(doc: Document) -> (String, String, String, String) {
    //get article meta data
    let art_meta = doc
        .find(Class("article-label"))
        .next();
    let meta_data;
    if let Some(val) = art_meta {
        meta_data = String::from(val.text().trim());
    } else {
        meta_data = String::from("No - Metadata");
    }
    //get author name
    let auth = doc.find(Class("bioLink")).next();
    let author_name;
    if let Some(val) = auth {
        author_name = val.text()
    } else {
        author_name = String::from("No - Author");
    }
    //get article publish date
    let date = doc.find(Class("date")).next();
    let publish_date;
    if let Some(val) = date {
        publish_date = String::from(val.text().trim());
    } else {
        publish_date = String::from("No - Date");
    }
    //get article text
    let text = doc.find(Class("articleBody")).next();
    let mut article_text: String = String::from("");
    if let Some(val) = text {
        //for each element P inside of the article body
        // push the trimmed text value onto article_text
        for x in val.find(Name("p")) {
            article_text.push_str(&x.text().trim_start());
        }
    } else {
        article_text = String::from("No - Article Text")
    }
    //respond with a tuple which can then be parsed by 
    // the News struct.
    (meta_data, author_name, publish_date, article_text)
}