use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::store;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("id");
    let is_2 = params.contains_key("old_pwd");
    let is_3 = params.contains_key("new_pwd");
    if is_1 && is_2 && is_3 {
        let id = params.get("id").unwrap();
        let old_pwd = params.get("old_pwd").unwrap();
        let new_pwd = params.get("new_pwd").unwrap();
        let res_old = store::auth_id_pwd(old_pwd, id);
        if res_old.len() == 1 {
            let res_new = store::change_pwd(new_pwd, id);
            JsonRes::new(0, "success".to_string(), res_new)
        } else {
            JsonRes::new(-2, "old pwd error".to_string(), "")
        }
    } else {
        JsonRes::new(-1, "param error".to_string(), "")
    }
}