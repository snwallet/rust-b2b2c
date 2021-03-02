
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::category;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;

    let is_1 = params.contains_key("id");
    if is_1 {
        let id = params.get("id").unwrap();
        let res = category::delete(id);
        JsonRes::new(0, "success".to_string(), res)
    }else {
        JsonRes::new(-1, "param error".to_string(), "")
    }
}