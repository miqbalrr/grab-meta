mod meta_error;
mod site;
mod meta;

use select::document::Document;
use site::{Site, Website, find_manual_meta, find_og_meta, find_tw_meta};

pub fn get_meta(url: &str) -> Result<meta::Meta, meta_error::MetaError>{ 
    let mut site = Site::new(url);
    site.check_type()?;
    let doc = Document::from(site.content.as_ref());
    let default_meta = meta::Meta::new("kosong", "kosong", "kosong");
    let result: meta::Meta =  match site.meta_type {
        meta::MetaType::Og => find_og_meta(&doc).unwrap_or(default_meta),
        meta::MetaType::Tw => find_tw_meta(&doc).unwrap_or(default_meta),
        meta::MetaType::Manual => find_manual_meta(&doc).unwrap_or(default_meta),
        meta::MetaType::Facebook => find_manual_meta(&doc).unwrap_or(default_meta),
        meta::MetaType::Twitter => return Err(meta_error::MetaError::SomeError("twitter site is not available at this momment")),
        meta::MetaType::Instagram => find_og_meta(&doc).unwrap_or(default_meta),
    };
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_meta_test() {
        let mymeta = get_meta("https://github.com/miqbalrr/grab-meta").unwrap();
        assert_eq!(mymeta.title, "miqbalrr/grab-meta");
    }
}
