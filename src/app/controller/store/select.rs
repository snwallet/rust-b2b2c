
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::model::store;
use crate::app::lib::param;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    let is_1 = params.contains_key("store_title");

    if is_1 {
        let store_title = params.get("store_title").unwrap();
        let data = store::select_store_title(store_title);
        JsonRes::new(0,"success".to_string(),data)
    }else{
        let data = store::select_all();
        JsonRes::new(0,"success".to_string(),data)
    }
}