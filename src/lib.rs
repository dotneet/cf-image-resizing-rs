use hex;
use hex::ToHex;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::thread;
use std::time;

use worker::*;

use manipulation::*;

mod manipulation;
mod utils;

fn get_query_params(url: Url) -> Result<HashMap<String, String>> {
    Ok(url.query_pairs().into_owned().collect())
}

async fn fetch_image(src: &str) -> Result<Response> {
    let request = Request::new(src, Method::Get);
    Fetch::Request(request?).send().await
}

fn make_image_response(image_bytes: Vec<u8>, format: String) -> Result<Response> {
    let len = image_bytes.len();
    let response = Response::from_bytes(image_bytes)?;
    let mut headers = Headers::new();
    headers.set("content-type", format!("image/{}", format).as_str())?;
    headers.set("cache-control", "s-maxage=10")?;
    headers.set("content-length", format!("{}", len).as_str())?;
    Ok(response.with_headers(headers))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    router
        .get("/favicon.ico", |_, _| Response::error("not found", 404))
        .get_async("/", |req, _| async move {
            let cache = Cache::open("whatever".to_string()).await;
            let query_params = get_query_params(req.url()?)?;
            let src = query_params.get("src").ok_or("error".to_owned())?;
            let manipulation = ManipulationParams::from_hash_map(&query_params);

            let url = req.url().unwrap().to_string();
            let cache_key = Request::new("https://example.com/hoge2", req.method()).unwrap();
            let result_cache = cache.get(&cache_key, true).await.unwrap();
            if result_cache.is_some() {
                console_log!("result cache hit");
                let data = result_cache.unwrap().bytes().await.unwrap();
                let h = data.encode_hex::<String>();
                console_log!("result cache hit: {} ", &h[..32]);
                // return Ok(result_cache.unwrap());
                // return Response::ok("hoge");
                return Ok(make_image_response(data, manipulation.format).unwrap());
            } else {
                console_log!("result cache miss: {}", url);
                // let src_cache = cache.get(src, false).await.unwrap();
                let src_cache: Option<Response> = None;
                let image_input: Vec<u8> = match src_cache {
                    Some(mut res) => {
                        console_log!("src cache hit");
                        res.bytes().await.unwrap()
                    }
                    None => {
                        console_log!("src cache miss");
                        let mut response = fetch_image(src).await?;
                        let result = response.bytes().await?;
                        // cache.put(src, response).await;
                        result
                    }
                };

                let image_output_1 = manipulation.apply(&image_input)?;
                let image_output_2 = manipulation.apply(&image_input)?;
                let response =
                    make_image_response(image_output_1.clone(), manipulation.format.clone())
                        .unwrap();
                let response_for_cache =
                    make_image_response(image_output_2.clone(), manipulation.format.clone())
                        .unwrap();
                match cache.put(&cache_key, response).await {
                    Ok(_) => {
                        console_log!("cached: {}", &hex::encode(image_output_1)[0..32])
                    }
                    _ => console_log!("cache error"),
                }
                Ok(response_for_cache)
            }
        })
        .run(req, env)
        .await
}
