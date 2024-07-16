use url::Url;

#[derive(Clone, Debug)]
pub struct Config {
    pub api_root: Url,
    pub api_token: Option<String>,
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.api_root == other.api_root && self.api_token == other.api_token
    }
}

impl Eq for Config {}
