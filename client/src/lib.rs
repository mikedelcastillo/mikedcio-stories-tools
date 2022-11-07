use surf::{Client, Config, Url};

const BASE_PATH: &str = "path/";

pub fn get_url(path: &str) -> String {
    format!("{}/{}", BASE_PATH, path)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn try_surf() -> String {
    let url = Url::parse("http://localhost:3000").unwrap();
    let client: Client = Config::new().set_base_url(url).try_into().unwrap();

    let mut req = client.get("http://localhost:3000/api/hello").await.unwrap();
    let body = req.body_string().await;
    body.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
