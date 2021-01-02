use chrono::{
    DateTime,
    Utc,
};
/// ## NewsEnum
/// A Enum which can be used to select a NewsSite to scrape.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum NewsEnum{
    CTV,
    CBC,
    GlobalNews,
}

#[derive(Clone, Debug)]
pub struct AllNews {
    ctv: Vec<News>,
    cbc: Vec<News>,
    global: Vec<News>,
}

#[allow(dead_code)]
impl AllNews {
    pub fn new(    
        ctv: Vec<News>,
        cbc: Vec<News>,
        global: Vec<News>
    ) -> AllNews{
        AllNews {
            ctv: ctv,
            cbc: cbc,
            global: global
        }
    }
    pub fn get_ctv(&self) -> &Vec<News> {
        &self.ctv
    }
    pub fn get_cbc(&self) -> &Vec<News> {
        &self.cbc
    }
    pub fn get_global(&self) -> &Vec<News> {
        &self.global
    }
}

/// ## NewsSite
/// This struct is used to get the a NewsSite for an associated
/// news company.
#[non_exhaustive]
pub struct NewsSite;

impl NewsSite {
    pub fn get_site(news: &NewsEnum) -> String{
        match news {
            NewsEnum::CBC => String::from("https://www.cbc.ca/"),
            NewsEnum::CTV => String::from("https://www.ctvnews.ca/"),
            NewsEnum::GlobalNews => String::from("https://globalnews.ca/"),
        }
    }
}
/// ## News
/// Main data structure of the program that pulled information
/// is placed.
#[derive(Clone, Debug)]
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


impl News {
    pub fn new(
        news_enum: NewsEnum,
        article_link: String,
        img_link: String,
        title: String,
        desc: String,
        article_text: String,
        author: String,
        metadata: String,
    ) -> News {
        let scrape_date = Utc::now();
        let article_date = String::from("");
        let news_site = NewsSite::get_site(&news_enum);
        News {
            news_enum,
            news_site,
            article_link,
            img_link,
            title,
            desc,
            author,
            metadata,
            article_text,
            article_date,
            scrape_date
        }
    }
    pub fn add_scrape_article(&mut self, missing_data: (String, String, String, String)){
        match self.news_enum {
            NewsEnum::CBC => {
                let (author, time, desc, text) = missing_data;
                self.author = author;
                self.article_date = time;
                self.desc = desc;
                self.article_text = text;
            },
            NewsEnum::CTV => {
                let (meta, author, date, text) = missing_data;
                self.author = author;
                self.metadata = meta;
                self.article_text = text;
                self.article_date = date;
            },
            NewsEnum::GlobalNews => {
                let (author, text, date, desc) = missing_data;
                self.author = author;
                self.desc = desc;
                self.article_text = text;
                self.article_date = date;
            }
        }
    }
}