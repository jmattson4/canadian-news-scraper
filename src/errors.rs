use std::fmt;

type Result<T> = std::result::Result<T, ScrapeError>;

#[derive(Debug, Clone)]
pub struct ScrapeError;

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There was an issue scraping a website.")
    }
}
