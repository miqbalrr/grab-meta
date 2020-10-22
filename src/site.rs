use crate::meta_error;
use crate::meta;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Name};

/*
* site here is only has url property
*/
#[derive(Clone)]
pub struct Site<'a>{
    pub url: &'a str,
    pub content: String,
    pub meta_type: meta::MetaType,
    pub is_sosmed: bool
}

/*
* interface fo website
*/
pub trait Website{
    fn get_html(&mut self) -> Result<(), meta_error::MetaError>;
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
        (meta::MetaType::Og, false)
    }
}

impl<'a> Site<'a>{
    #[allow(dead_code)]
    pub fn new(url: &'a str) -> Self {
        let check_url = is_sosmed_site(url);
        Site{url, content: String::new(), meta_type: check_url.0, is_sosmed: check_url.1}
    }
}

impl<'a> Website for Site<'a>{
    /*
    * Get content
    * MetaError contains Request Error for reqwest
    * update self content
    */
    #[tokio::main]
    async fn get_html(&mut self) -> Result<(), meta_error::MetaError> {
       let client = reqwest::Client::new();
       let res = client
            .get(self.url)
            .header("USER_AGENT", "reqwest")
            .send().await?.text().await?;
       if !res.trim().is_empty(){
        self.content = res;
        return Ok(())
       } 
       Err(meta_error::MetaError::SomeError("Somehow error request"))
    }

    /*
    *
    * Check Type
    * update self meta_type
    */
    fn check_type(&mut self) -> Result<(), meta_error::MetaError> {
        self.get_html()?;
        if self.is_sosmed {
            return Ok(())
        }
        let doc = Document::from(self.content.as_str());

        for my_type in meta::MetaType::Og.into_iter() {
           match my_type {
                meta::MetaType::Og => {
                   if doc.find(Attr("property","og:title")).count() > 0 {
                       self.meta_type = meta::MetaType::Og;
                       return Ok(()) 
                   }
                },
                meta::MetaType::Tw => {
                    if doc.find(Attr("name", "twitter:title")).count() > 0 {
                        self.meta_type = meta::MetaType::Tw;
                        return Ok(()) 
                    }
                },
                _ => {
                    self.meta_type = meta::MetaType::Manual;
                    return Ok(()) 
                },
            };
        }
        Err(meta_error::MetaError::SomeError("no type"))
    }
}

/*
*
* Find meta with OG type
* params = doc type Document
*/
pub fn find_og_meta(doc: &Document) -> Option<meta::Meta> {
    let title: String = doc.find(Attr("property","og:title")).filter_map(|x| x.attr("content")).collect();
    let description: String = doc.find(Attr("property","og:description")).filter_map(|x| x.attr("content")).collect();
    let thumbnail: String = doc.find(Attr("property","og:image")).filter_map(|x| x.attr("content")).collect();
    let meta = meta::Meta::new(title.as_ref(), description.as_ref(), thumbnail.as_ref());
    Some(meta)
}

/*
*
* Find meta with Tw type
* params = doc type Document
*/
pub fn find_tw_meta(doc: &Document) -> Option<meta::Meta> {
    let title: String = doc.find(Attr("name","twitter:title")).filter_map(|x| x.attr("content")).collect();
    let description: String = doc.find(Attr("name","twitter:description")).filter_map(|x| x.attr("content")).collect();
    let thumbnail: String = doc.find(Attr("name","twitter:image")).filter_map(|x| x.attr("content")).collect();
    let meta = meta::Meta::new(title.as_ref(), description.as_ref(), thumbnail.as_ref());
    Some(meta)
}

/*
*
* Find meta with Manual type
* params = doc type Document
*/
pub fn find_manual_meta(doc: &Document) -> Option<meta::Meta> {
    let title: String = doc.find(Name("title")).next().unwrap().text();
    let nodedsc = doc.find(Name("p")).next();
    let thumbnail: String = doc.find(Name("img")).take(1).filter_map(|x| x.attr("src")).collect();
    let mut description = String::from("No Desc Available");
    if !nodedsc.is_none() {
        description = nodedsc?.text();
    }
    let meta = meta::Meta::new(title.as_ref(), description.as_ref(), thumbnail.as_ref());
    Some(meta)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_html_content() {
        // i test with mozilla site just for the content 
        // let mut site = Site::new("http://detectportal.firefox.com/success.txt");
        // // let mut site = Site::new("https://m.facebook.com/story.php?story_fbid=1695480023942518&id=100004416098392&sfnsn=wiwspwa&extid=vGtMhEVlL3ZOL2Nd");
        // // println!("{:?}",site.content);
        // site.get_html().expect("error");
        // assert_eq!("success\n", site.content)
    }

    #[test]
    fn check_type() { 
        // meta should type facebook or twitter
        let mut site1 = Site::new("https://facebook.com");
        site1.check_type().expect("error");
        assert_eq!(meta::MetaType::Facebook, site1.meta_type);

        let mut site2 = Site::new("https://twitter.com");
        site2.check_type().expect("error");
        assert_eq!(meta::MetaType::Twitter, site2.meta_type);

        let mut site2 = Site::new("https://www.instagram.com/p/CFs2jU5nbTS/");
        site2.check_type().expect("error");
        assert_eq!(meta::MetaType::Instagram, site2.meta_type)
        // assert_eq!(3, 4)
    }

    #[test]
    fn check_type_manual() {
        let mut site = Site::new("http://iqbalcakep.com");
        site.check_type().expect("error");
        assert_eq!(meta::MetaType::Manual, site.meta_type)
    }

    #[test]
    fn check_type_og() {
        let mut site = Site::new("https://github.com/");
        site.check_type().expect("error");
        assert_eq!(meta::MetaType::Og, site.meta_type)
    }
}
