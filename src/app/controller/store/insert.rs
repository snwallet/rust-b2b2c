
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::store;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("store_title");
    let is_2 = params.contains_key("store_name");
    let is_3 = params.contains_key("store_pwd");
    let is_4 = params.contains_key("store_desc");

    if is_1 && is_2 && is_3 && is_4  {
        let store_title = params.get("store_title").unwrap();
        let store_name = params.get("store_name").unwrap();
        let store_pwd = params.get("store_pwd").unwrap();
        let store_desc = params.get("store_desc").unwrap();
        //检查店铺名称 登录用户名是否重复
        let store_title_bool = store::allow_store_title(store_title);
        let store_name_bool = store::allow_store_name(store_name);

        match(store_title_bool,store_name_bool){
            (true,true)=>{
                let insert_id = store::insert(store_title,store_name,store_pwd,store_desc);
                JsonRes::new(0,"success".to_string(),insert_id)
            },
            (true,false)=>JsonRes::new(-4,"store title is exist".to_string(),""),
            (false,true)=>JsonRes::new(-3,"store name is exist".to_string(),""),
            (false,false)=>JsonRes::new(-2,"store title and name is exist".to_string(),"")

        }
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}