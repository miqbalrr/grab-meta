struct Site<T>{
    url: T
}

trait Url{
    fn get_content(&self) -> &'static str;
}

impl Site<&'static str>{
    fn new(url: &'static str) -> Self {
        Site {url}
    }
}

impl Url for Site<&'static str>{
    fn get_content(&self) -> &'static str {
       self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_content_url() {
        let site: Site<&'static str> = Site::new("https://iqbalcakep.com");
        assert_eq!(site.url, "https://iqbalcakep.com")
    }
}


