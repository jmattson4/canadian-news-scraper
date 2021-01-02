use select::document::Document;
use select::predicate::{Class, Name};
use crate::news::{NewsEnum, News};
use crate::scraper;

/// ## scrape_global_news
/// This function scrapes the front page of https://globalnews.ca/
pub async fn scrape_global_news(doc: Document, global: &NewsEnum) -> Vec<News> {
    let mut news_vec: Vec<News> = Vec::new();
    //the numbers seem arbitrary but the website has 2 blank articles
    // which take up space in the markup when pulling so it comes
    //out to 12 actual articles.
    let main = doc
        .find(Name("main"))
        .next()
        .unwrap();

    let top_fourteen = main.find(Class("c-posts__item")).take(14);

    for node in top_fourteen {
        //get title
        let title = node.find(Class("c-posts__headlineText")).next();
        let title_text: String;
        if let Some(val) = title {
            title_text = val.text();
        } else {
            title_text = String::from("No - Title");
        }
        // get article_link
        let art = node.find(Name("a")).next();
        let article_link: String;
        if let Some(val) = art {
            let a = val.attr("href");
            if let Some(val) = a {
                article_link = String::from(val);
            } else {
                article_link = String::from("No - Article Link");
            }
        } else {
            article_link = String::from("No - Article Link");
        }
        //get meta link
        let meta = node.find(Class("c-posts__about")).next();
        let meta_text: String;
        if let Some(val) = meta {
            meta_text = String::from(val.text().trim());
        } else {
            meta_text = String::from("No - Metadata");
        }
        let meta_text = clean_metadata_string(meta_text);
        if article_link != String::from("No - Article Link"){
            let news = News::new(
                global.clone(), 
                article_link,
                String::from(""), 
                title_text, 
                String::from(""), 
                String::from(""), 
                String::from(""), 
                meta_text);
            news_vec.push(news);
        }        
    }
    news_vec
}

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

async fn scrape_article(doc: Document) -> (String, String, String, String) {
    //get author
    let auth = doc.find(Class("c-byline__attribution")).next();
    let author: String;
    if let Some(val) = auth {
        author = String::from(val.text().trim());
    } else {
        author = String::from("No - Author");
    }

    //get article text
    let text = doc.find(Name("article")).next();
    let mut article_text: String = String::from("");
    if let Some(val) = text {
        for x in val.find(Name("p")) {
            article_text.push_str(&x.text().trim_start());
        }
    } else {
        article_text = String::from("No - Article Text")
    }

    //get date text
    let d = doc.find(Class("c-byline__dates")).next();
    let date: String;
    if let Some(val) = d {
        date = String::from(val.text().trim());
    } else {
        date = String::from("No - Date");
    }
    let date = clean_date_string(date);
    //println!("{}", author);
    println!("{}", date);
    ( author, article_text, date, String::from("") )
}

/// ## clean_date_string
/// this function is used to clean up the date
/// string that is pulled off the site a little bit
/// it could still probably use some work to make it look
/// cleaner
fn clean_date_string(s: String) -> String {
    let str_vec: Vec<char> = s.chars().collect();
    let mut new_vec: Vec<char> = Vec::new();
    let length = str_vec.len() - 1;
    for i in 0..length {
        if i == 0 {
            new_vec.push(str_vec[i])
        }
        if i >= 1 && i < length {
            if !str_vec[i - 1].is_whitespace() || !str_vec[i + 1].is_whitespace() {
                new_vec.push(str_vec[i])
            }           
        }
    }
    new_vec.push(str_vec[length]);
    let s = new_vec.iter().collect();
    s
}
/// ## clean_metadata_string
/// this function is used to clean up the meta data string
/// that is pulled off the site a little bit
/// it could still probably use some work to make it look
/// cleaner
fn clean_metadata_string(s: String) -> String{
    let mut str_vec: Vec<char> = s.chars().collect();
    //go through the char vector and only retain values
    // which dont jave \t char
    str_vec.retain(|c| {
        if *c == '\t'{
            false
        } else{
            true
        }
    });
    let s: String = str_vec.iter().collect();
    //replace all newlines with a space to make it more readable.
    let s = str::replace(&s, "\n", " ");
    s
}