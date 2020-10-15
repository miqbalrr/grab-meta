use crate::meta_error;
use crate::meta;
use reqwest;

/*
* site here is only has url property
*/
#[derive(Clone)]
pub struct Site{
    url: &'static str,
    content: String,
    meta_type: meta::MetaType,
    is_sosmed: bool
}

/*
* interface fo website
*/
pub trait Website{
    fn get_content(&mut self) -> Result<Self, meta_error::MetaError> where Self: std::marker::Sized;
    fn check_type(&mut self) -> Result<Self, meta_error::MetaError> where Self: std::marker::Sized;
}

/*
* implement site for generete new
*/
fn is_sosmed_site(url: &str) -> (meta::MetaType, bool) {
    if url.contains("facebook.com") {
        (meta::MetaType::Facebook, true)
    } else if url.contains("twitter.com") {
        (meta::MetaType::Twitter, true)
    } else {
        (meta::MetaType::Og, false)
    }
}

impl Site<>{
    pub fn new(url: &'static str) -> Self {
        let check_url = is_sosmed_site(url);
        Site {url, content: String::new(), meta_type: check_url.0, is_sosmed: check_url.1}
    }
}

impl Website for Site{

    /*
    * MetaError contains Request Error for reqwest
    * String result is html content
    */
    #[tokio::main]
    async fn get_content(&mut self) -> Result<Self, meta_error::MetaError> {
       let client = reqwest::Client::new();
       let res = client.get(self.url).send().await?.text().await?;
       Ok(Self{url: self.url, content: res, meta_type: meta::MetaType::Og, is_sosmed: self.is_sosmed})
    }

    fn check_type(&mut self) -> Result<Self, meta_error::MetaError> {
        Ok(Self{url: self.url, content: self.content.clone(), meta_type: meta::MetaType::Og, is_sosmed: self.is_sosmed})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_content() {
        // i test with mozilla site just for the content 
        let mut site = Site::new("http://detectportal.firefox.com/success.txt");
        let site = site.get_content().expect("error");
        assert_eq!("success\n", site.content)
    }

    #[test]
    fn check_type() { // meta should type facebook or twitter
        let site = Site::new("https:://facebook.com");
        assert_eq!(meta::MetaType::Facebook, site.meta_type);

        let site = Site::new("https:://twitter.com");
        assert_eq!(meta::MetaType::Twitter, site.meta_type)

    }
}
