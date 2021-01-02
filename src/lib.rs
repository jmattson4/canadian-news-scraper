/// # News
/// This module holds data structure, enums and logic
/// related to the newsites which are scraped in scraper.
mod news;
/// # Scraper
/// This module holds all related scraper
/// modules and logic.
/// 
mod scraper;
mod errors;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbc_scrape() {
        let news_site = news::NewsEnum::CBC;
        let val = scraper::scrape(news_site);
        assert_eq!(val.len(), 4);
    }
    #[test]
    fn ctv_scrape() {
        let news_site = news::NewsEnum::CTV;
        let val = scraper::scrape(news_site);
        assert_eq!(val.len(), 12);
    }
    #[test]
    fn global_news_scrape() {
        let news_site = news::NewsEnum::GlobalNews;
        let val = scraper::scrape(news_site);
        assert_eq!(val.len(), 12);
    }    
}
