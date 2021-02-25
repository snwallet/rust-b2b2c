
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::store;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("store_title");
    let is_2 = params.contains_key("store_name");
    let is_3 = params.contains_key("store_desc");
    let is_4 = params.contains_key("id");

    if is_1 && is_2 && is_3 && is_4  {
        let store_title = params.get("store_title").unwrap();
        let store_name = params.get("store_name").unwrap();
        let store_desc = params.get("store_desc").unwrap();
        let id = params.get("id").unwrap();

        match store::select_id(id).get(0) {
            Some(s)=>{
                println!("s:{:?}",*store_title);
                println!("s:{:?}",s.store_title);
                let store_title_bool = s.store_title==*store_title || store::allow_store_title(store_title);
                let store_name_bool = s.store_name==*store_name || store::allow_store_name(store_name);

                match (store_title_bool, store_name_bool) {
                    (true, true) => {
                        let res = store::update(store_title, store_name, store_desc, id);
                        JsonRes::new(0, "success".to_string(), res)
                    },
                    (true, false) => JsonRes::new(-5, "store title is exist".to_string(), ""),
                    (false, true) => JsonRes::new(-4, "store name is exist".to_string(), ""),
                    (false, false) => JsonRes::new(-3, "store title and name is exist".to_string(), "")
                }
            },
            None=>JsonRes::new(-2,"id error".to_string(),"")
        }
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}