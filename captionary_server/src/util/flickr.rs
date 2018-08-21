use curl::easy::Easy;
use serde_json::de;
use std::env;

pub struct Flickr {
}

#[derive(Deserialize, Debug)]
struct FlickrResponseContainer {
    photos: FlickrResponse,
    stat: String
}

#[derive(Deserialize, Debug)]
struct FlickrResponse {
    page: i32,
    pages: i32,
    perpage: i32,
    total: i32,
    photo: Vec<FlickrPhotoRecord>
}

#[derive(Deserialize, Debug)]
struct FlickrPhotoRecord {
    id: String,
    owner: String,
    secret: String,
    server: String,
    farm: u32,
    title: String,
    ispublic: u8,
    isfriend: u8,
    isfamily: u8,
}

impl FlickrPhotoRecord {
    fn to_string(&self) -> String {
        format!(
            "https://farm{}.staticflickr.com/{}/{}_{}.jpg",
            self.farm,
            self.server,
            self.id.to_string(),
            self.secret
        )
    }
}

impl Flickr {
    pub fn get_image_url() -> Option<String> {
        match Self::get_flickr_response() {
            Some(response) => Some(response.photos.photo[0].to_string()),
            None => None
        }
        
    }

    fn get_flickr_response() -> Option<FlickrResponseContainer> {
        let flickr_key = env::var("FLICKR_KEY").expect("Please set env var: FLICKR_KEY");
        let mut flickr_response = Vec::new();

        let url = format!(
            "https://api.flickr.com/services/rest/?method=flickr.interestingness.getList&api_key={}&format=json&nojsoncallback=1",
            flickr_key
        );

        Flickr::http_get(&mut flickr_response, &url);
        
        let res = String::from_utf8(flickr_response).unwrap();

        let json: Option<FlickrResponseContainer> = de::from_str(&res).ok();

        return json
    }

    fn http_get(response: &mut Vec<u8>, url_str: &String) {
        let mut easy = Easy::new();

        easy.url(url_str).unwrap();
        let mut transfer = easy.transfer();

        transfer
            .write_function(|data| {
                response.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
        transfer.perform().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn should_download_json() {
        dotenv().ok();

        let json = Flickr::get_flickr_response();
        assert!(json.is_some())
    }

    #[test]
    fn should_get_image_url() {
        dotenv().ok();

        assert!(Flickr::get_image_url().is_some());
    }
}
