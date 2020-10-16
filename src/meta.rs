pub struct Meta {
    title: String,
    description: String,
    thumbnail: String
}

#[derive(Clone, Debug, PartialEq)]
pub enum MetaType{
    // for og meta
    Og(String),
    // for twitter meta
    Tw(String),
    // for twitter site
    Twitter,
    // for facebook site
    Facebook,
    // for ig site / url
    Instagram,
    // for Manual 
    Manual(String)
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
            1 => Some(MetaType::Og("og:title".to_string())),
            2 => Some(MetaType::Tw("twitter:title".to_string())),
            3 => Some(MetaType::Manual("title".to_string())),
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