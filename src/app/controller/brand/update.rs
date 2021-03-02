
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::brand;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
    //insert(store_title:String, store_name:String, store_pwd:String, store_desc:String)
    let is_1 = params.contains_key("brand_name");
    let is_2 = params.contains_key("brand_logo");
    let is_3 = params.contains_key("brand_url");
    let is_4 = params.contains_key("sort");
    let is_5 = params.contains_key("id");

    if is_1 && is_2 && is_3 && is_4 && is_5  {
        let brand_name = params.get("brand_name").unwrap();
        let brand_logo = params.get("brand_logo").unwrap();
        let brand_url = params.get("brand_url").unwrap();
        let sort = params.get("sort").unwrap();
        let id = params.get("id").unwrap();

        match brand::select_id(id).get(0) {
            Some(s)=>{
                println!("s:{:?}",*brand_name);
                println!("s:{:?}",s.brand_name);
                let brand_name_bool = s.brand_name==*brand_name || brand::allow_brand_name(brand_name);
                match brand_name_bool {
                    true => {
                        let res = brand::update(brand_name, brand_logo, brand_url,sort, id);
                        JsonRes::new(0, "success".to_string(), res)
                    },
                    _ => JsonRes::new(-5, "brand name is exist".to_string(), ""),
                }
            },
            None=>JsonRes::new(-2,"id error".to_string(),"")
        }
    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}