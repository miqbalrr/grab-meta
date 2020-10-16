use crate::meta_error;
use crate::meta;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name};

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
trait Website{
    fn get_content(&mut self) -> Result<(), meta_error::MetaError>;
    fn check_type(&mut self) -> Result<(), meta_error::MetaError>;
}

/*
* implement site for generete new
*/
#[allow(dead_code)]
// replacing site meta_type
fn is_sosmed_site(url: &str) -> (meta::MetaType, bool) {
    if url.contains("facebook.com") {
        (meta::MetaType::Facebook, true)
    } else if url.contains("twitter.com") {
        (meta::MetaType::Twitter, true)
    } else if url.contains("instagram.com") {
        (meta::MetaType::Instagram, true)
    } else {
        (meta::MetaType::Og("og:title".to_string()), false)
    }
}

impl Site{
    #[allow(dead_code)]
    pub fn new(url: &'static str) -> Self {
        let check_url = is_sosmed_site(url);
        Site {url, content: String::new(), meta_type: check_url.0, is_sosmed: check_url.1}
    }
}

impl Website for Site{

    /*
    * Get content
    * MetaError contains Request Error for reqwest
    * update self content
    */
    #[tokio::main]
    async fn get_content(&mut self) -> Result<(), meta_error::MetaError> {
       let client = reqwest::Client::new();
       let res = client.get(self.url).send().await?.text().await?;
       self.content = res;
       Ok(())
    }

    /*
    *
    * Check Type
    * update self meta_type
    */
    fn check_type(&mut self) -> Result<(), meta_error::MetaError> {
        if self.meta_type == meta::MetaType::Facebook || 
        self.meta_type == meta::MetaType::Twitter || self.meta_type == meta::MetaType::Instagram {
           return Ok(()) 
        }
        self.get_content();
        let doc = Document::from(self.content.as_str());

        for my_type in self.meta_type.clone().into_iter() {
            match my_type {
                meta::MetaType::Og(attr) => {
                   if doc.find(Attr("property", attr.as_str())).count() > 0 {
                       self.meta_type = meta::MetaType::Og(attr);
                       return Ok(()) 
                   }
                },
                meta::MetaType::Tw(attr) => {
                    if doc.find(Attr("name", attr.as_str())).count() > 0 {
                        self.meta_type = meta::MetaType::Tw(attr);
                        return Ok(()) 
                    }
                },
                _ => {
                    self.meta_type = meta::MetaType::Manual("title".to_string());
                    return Ok(()) 
                },
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_content() {
        // i test with mozilla site just for the content 
        let mut site = Site::new("http://detectportal.firefox.com/success.txt");
        site.get_content().expect("error");
        assert_eq!("success\n", site.content)
    }

    #[test]
    fn check_type() { 
        // meta should type facebook or twitter
        let mut site1 = Site::new("https://facebook.com");
        site1.check_type().expect("error");
        assert_eq!(meta::MetaType::Facebook, site1.meta_type);

        let mut site2 = Site::new("https:://twitter.com");
        site2.check_type().expect("error");
        assert_eq!(meta::MetaType::Twitter, site2.meta_type);

        let mut site2 = Site::new("https:://instagram.com");
        site2.check_type().expect("error");
        assert_eq!(meta::MetaType::Instagram, site2.meta_type)
    }

    #[test]
    fn check_type_manual() {
        let mut site = Site::new("http://iqbalcakep.com");
        site.get_content();
        site.check_type().expect("error");
        assert_eq!(meta::MetaType::Manual("title".to_string()), site.meta_type)
    }
}
