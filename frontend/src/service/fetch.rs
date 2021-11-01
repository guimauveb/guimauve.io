use {
    wasm_bindgen::{JsCast, JsValue},
    wasm_bindgen_futures::JsFuture,
    web_sys::{/*console,*/ Request, RequestInit, RequestMode, RequestRedirect, Response},
    yew::web_sys,
};

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    DELETE,
}

// TODO - Check payload type
pub async fn fetch(
    url: String,
    method: String,
    payload: Option<String>,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(&method);
    opts.mode(RequestMode::Cors);
    opts.redirect(RequestRedirect::Follow);

    match payload {
        Some(content) => opts.body(Some(&JsValue::from_str(&content))),
        None => opts.body(None),
    };

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set(
        "Access-Control-Request-Headers",
        "Content-Type, Authorization",
    )?;
    request
        .headers()
        .set("Access-Control-Request-Method", &method)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert JS Promise into a Rust Future
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub struct Fetch();

impl Fetch {
    async fn fetch(
        url: String,
        method: Method,
        payload: Option<String>,
    ) -> Result<JsValue, JsValue> {
        let method = match method {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::OPTIONS => "OPTIONS",
            Method::DELETE => "DELETE",
        };
        fetch(url, method.to_string(), payload).await
    }

    pub async fn get(url: String) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::GET, None).await
    }

    pub async fn patch(url: String, payload: Option<String>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::PATCH, payload).await
    }

    pub async fn post(url: String, payload: Option<String>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::POST, payload).await
    }

    pub async fn put(url: String, payload: Option<String>) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::PUT, payload).await
    }

    pub async fn delete(url: String) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::DELETE, None).await
    }
}
