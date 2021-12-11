use std::collections::HashMap;

use worker::*;

use manipulation::*;

mod manipulation;
mod utils;

fn get_query_params(url: Url) -> Result<HashMap<String, String>> {
    Ok(url.query_pairs().into_owned().collect())
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
            let src = query_params.get("src").ok_or("error")?;
            let manipulation = ManipulationDefinition::from_hash_map(&query_params);

            let image_input = fetch_image(src).await?;
            let image_output = manipulation.modify_image(&image_input)?;

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
