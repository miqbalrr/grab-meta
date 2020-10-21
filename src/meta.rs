use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub description: String,
    pub thumbnail: String
}

#[derive(Clone, Debug, PartialEq)]
pub enum MetaType{
    // for og meta
    Og,
    // for twitter meta
    Tw,
    // for twitter site
    Twitter,
    // for facebook site
    Facebook,
    // for ig site / url
    Instagram,
    // for Manual 
    Manual
}

#[derive(Debug, Clone)]
pub struct MetaIterator {
    meta: MetaType,
    next: u32
}

impl Iterator for MetaIterator {
    type Item = MetaType;
    fn next(&mut self) -> Option<Self::Item> {
        self.next += 1;
        match self.next {
            1 => Some(MetaType::Og),
            2 => Some(MetaType::Tw),
            3 => Some(MetaType::Manual),
            _ => None
        }
    }
}

impl IntoIterator for MetaType {
    type IntoIter = MetaIterator;
    type Item = MetaType;
    fn into_iter(self) -> Self::IntoIter {
        MetaIterator{ meta: self , next: 0 }
    } 
}


impl Meta {
    pub fn new(title: &str, description: &str, thumbnail: &str ) -> Self {
        Meta {title: title.to_string() , description: description.to_string(), thumbnail: thumbnail.to_string()}
    }
}