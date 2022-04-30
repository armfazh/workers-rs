use crate::request::Request;
use crate::response::Response;
use crate::{Headers, Result, RouteContext};
use worker_sys::ASSET_MANIFEST;

const ASSET_NAMESPACE: &str = "__STATIC_CONTENT";

pub async fn get_asset_from_kv<D>(req: Request, ctx: RouteContext<D>) -> Result<Response> {
    let url = req.url()?;
    let path = &url.path()[1..];
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let key = ASSET_MANIFEST.get(path).unwrap_or(&path);
    let bytes = ctx.kv(ASSET_NAMESPACE)?.get(key).bytes().await?;
    if let Some(bytes) = bytes {
        let mut headers = Headers::new();
        headers.set("content_type", &mime.to_string())?;
        return Ok(Response::from_bytes(bytes)?.with_headers(headers));
    }

    Response::error("Not found", 404)
}
