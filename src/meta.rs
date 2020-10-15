pub struct Meta {
    title: String,
    description: String,
    thumbnail: String
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
    // for Manual 
    Manual
}

impl Meta {
    pub fn new(title: &str, description: &str, thumbnail: &str ) -> Self {
        Meta {title: title.to_string() , description: description.to_string(), thumbnail: thumbnail.to_string()}
    }
}