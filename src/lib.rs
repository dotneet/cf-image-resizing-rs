use std::collections::HashMap;

use image::imageops::FilterType;
use image::*;
use worker::*;

mod utils;

struct ManipulationParams {
    width: u32,
    height: u32,
    format: String,
}

impl ManipulationParams {
    pub fn new() -> Self {
        Self::with_data()
    }
    pub fn with_data() -> Self {
        Self {
            width: 200,
            height: 200,
            format: "png".to_string(),
        }
    }
    pub fn apply(&self, bytes: &Vec<u8>) -> Result<Vec<u8>> {
        let img = load_from_memory(&bytes).unwrap();
        let modified_image = img.resize_exact(self.width, self.height, FilterType::Gaussian);
        let mut dst: Vec<u8> = Vec::new();
        let image_format: ImageOutputFormat = match self.format.as_ref() {
            "png" => ImageOutputFormat::Png,
            _ => ImageOutputFormat::Jpeg(80),
        };
        modified_image.write_to(&mut dst, image_format).unwrap();
        Ok(dst)
    }
}

fn get_query_params(url: Url) -> Result<HashMap<String, String>> {
    Ok(url.query_pairs().into_owned().collect())
}

fn extract_manipulation_params(params: &HashMap<String, String>) -> Result<ManipulationParams> {
    let mut options = ManipulationParams::new();
    options.width = params
        .get("w")
        .ok_or("no width")?
        .parse::<u32>()
        .unwrap_or(100);
    options.height = params
        .get("h")
        .ok_or("no height")?
        .parse::<u32>()
        .unwrap_or(100);
    options.format = params
        .get("fmt")
        .unwrap_or(&String::from("png"))
        .to_string();
    Ok(options)
}

async fn fetch_image(src: &str) -> Result<Vec<u8>> {
    let request = Request::new(src, Method::Get);
    let response = Fetch::Request(request?).send().await;
    response.unwrap().bytes().await
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get_async("/", |req, _| async move {
            let query_params = get_query_params(req.url()?)?;
            let src = query_params.get("src").ok_or("error".to_owned())?;
            let manipulation = extract_manipulation_params(&query_params)?;

            let image_input = fetch_image(src).await?;
            let image_output = manipulation.apply(&image_input)?;

            let response = Response::from_bytes(image_output)?;
            let mut headers = Headers::new();
            headers.set(
                "content-type",
                format!("image/{}", manipulation.format).as_str(),
            )?;
            Ok(response.with_headers(headers))
        })
        .run(req, env)
        .await
}
