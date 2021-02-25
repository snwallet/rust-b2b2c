
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::store;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    let is_1 = params.contains_key("name");
    let is_2 = params.contains_key("pwd");
    if is_1 && is_2 {
        let name = params.get("name").unwrap();
        let pwd = params.get("pwd").unwrap();
        let login_user = store::login(name,pwd);
        if login_user.len()==1 {
            JsonRes::new(0,"success".to_string(),login_user)
        }else{
            JsonRes::new(-2,"name error or pwd error".to_string(),"")
        }
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }

}