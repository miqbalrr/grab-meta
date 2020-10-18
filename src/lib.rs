mod meta_error;
mod site;
mod meta;

use select::document::Document;
use site::{Site, Website, find_manual_meta, find_og_meta, find_tw_meta};

fn get_meta(url: &'static str) -> Result<meta::Meta, meta_error::MetaError> {
    let mut site = Site::new(url);
    site.check_type()?;
    let doc = Document::from(site.content.as_ref());
    let result: meta::Meta =  match site.meta_type {
        meta::MetaType::Og => find_og_meta(&doc).unwrap(),
        meta::MetaType::Tw => find_tw_meta(&doc).unwrap(),
        meta::MetaType::Manual => find_manual_meta(&doc).unwrap(),
        meta::MetaType::Facebook => find_og_meta(&doc).unwrap(),
        meta::MetaType::Twitter => find_og_meta(&doc).unwrap(),
        meta::MetaType::Instagram => find_og_meta(&doc).unwrap(),
    };
    println!("{:?}", result);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_meta_test() {
        let mymeta = get_meta("https://github.com/miqbalrr/grab-meta").expect("error");
        assert_eq!(mymeta.title, "miqbalrr/grab-meta");
    }
}
