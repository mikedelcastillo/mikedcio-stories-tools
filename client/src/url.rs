use std::env;

pub fn get_bucket_url(file_name: &String) -> String {
    let bucket_url = match env::var("BUCKET_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("BUCKET_URL is not set in the environment.");
            String::new()
        }
    };

    format!("{}{}", bucket_url, file_name)
}

pub fn get_api_url(endpoint: &String) -> String {
    let api_url = match env::var("API_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("API_URL is not set in the environment.");
            String::new()
        }
    };

    format!("{}api/{}", api_url, endpoint)
}
