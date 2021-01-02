/// # News
/// This module holds data structure, enums and logic
/// related to the newsites which are scraped in scraper.
mod news;
/// # Scraper
/// This module holds all related scraper
/// modules and logic.
mod scraper;

use scraper::{scrape, scrape_all};
use news::{NewsEnum, News};


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cbc_scrape() {
        let news_site = NewsEnum::CBC;
        let val = scrape(news_site);
        assert_eq!(val.len(), 4);
    }
    #[test]
    fn ctv_scrape() {
        let news_site = NewsEnum::CTV;
        let val = scrape(news_site);
        assert_eq!(val.len(), 12);
    }
    #[test]
    fn global_news_scrape() {
        let news_site = NewsEnum::GlobalNews;
        let val = scrape(news_site);
        assert_eq!(val.len(), 12);
    }
    #[test]
    fn all_scrape() {
        let val = scrape_all();
        assert_eq!(val.get_cbc().len(), 4);
        assert_eq!(val.get_ctv().len(), 12);
        assert_eq!(val.get_global().len(), 12);
    }       
}
