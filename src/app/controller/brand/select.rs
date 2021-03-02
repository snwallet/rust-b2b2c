
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::model::brand;
use crate::app::lib::param;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    let is_1 = params.contains_key("id");

    if is_1 {
        let id = params.get("id").unwrap();
        let data = brand::select_id(id);
        JsonRes::new(0,"success".to_string(),data)
    }else{
        let data = brand::select_all();
        JsonRes::new(0,"success".to_string(),data)
    }
}