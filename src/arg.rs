use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SubjectBackendQueryArg {
    pub page: Option<u32>,
    pub keyword: Option<String>,
    pub msg: Option<String>,
    pub is_del: Option<bool>,
}
impl SubjectBackendQueryArg {
    pub fn page(&self) -> u32 {
        match &self.page {
            Some(p) => *p,
            None => 0,
        }
    }
    pub fn keyword(&self) -> &str {
        match &self.keyword {
            Some(s) => s,
            None => "",
        }
    }
    pub fn is_del(&self) -> bool {
        match &self.is_del {
            Some(d) => *d,
            None => false,
        }
    }
}