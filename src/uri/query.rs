use std::collections::HashMap;
use regex::Regex;
use std::ops::Index;

#[derive(Debug, Eq, PartialEq)]
pub struct Query {
    pub query: HashMap<String, String>
}

impl Query {
    pub fn new(uri: &str) -> Option<Self> {
        let mut queries = HashMap::new();
        let re = Regex::new("\\?\\S+").unwrap();
        let query_string = match re.captures(uri) {
            Some(caps) => caps.index(0 as usize).to_string(),
            None => return None,
        };
        for key_value in query_string[1..].split('&') {
            let q: Vec<&str> = key_value.split('=').collect();
            queries.insert(q[0].to_string(), q[1].to_string());
        }
        Some(Query{query: queries})
    }

    // pub fn query(&self) -> Option<HashMap<String, String>> {
    //     if self.query.is_empty() {
    //         return None;
    //     }
    //     Some(*self.query)
    // }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_new_query() {
        let uri = "http://terassyi.net/hoge/fuga?query1=test1&query2=test2";
        let mut wanted = HashMap::new();
        wanted.insert("query1".to_string(),"test1".to_string());
        wanted.insert("query2".to_string(), "test2".to_string());
        assert_eq!(super::Query::new(uri), Some(super::Query{query: wanted}));
    }
    #[test]
    fn test_new_empy_query() {
        let uri = "http://terassy.net/hoge/fuga/index.html";
        assert_eq!(super::Query::new(uri), None);
    }
}