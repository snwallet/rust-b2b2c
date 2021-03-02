
use crate::app::lib::json_res::JsonRes;
use hyper::{Request, Body, Response};
use crate::app::lib::param;
use crate::app::model::category;

pub async fn main(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let params = param::post_param(req).await?;
  
    let is_1 = params.contains_key("cate_name");
    let is_2 = params.contains_key("parent_id");
    let is_3 = params.contains_key("level");
    let is_4 = params.contains_key("path");
    let is_5 = params.contains_key("sort");
    let is_6 = params.contains_key("is_on");
    let is_7 = params.contains_key("picture");

    if is_1 && is_2 && is_3 && is_4 && is_5 && is_6 && is_7 {
        let cate_name = params.get("cate_name").unwrap();
        let parent_id = params.get("parent_id").unwrap();
        let level = params.get("level").unwrap();
        let path = params.get("path").unwrap();
        let sort = params.get("sort").unwrap();
        let is_on = params.get("is_on").unwrap();
        let picture = params.get("picture").unwrap();
      
        //检查类别的名称是否重复
        let cate_name_bool = category::allow_category_name(cate_name);
        if cate_name_bool
        {
            let insert_id = category::insert(cate_name,parent_id,level,path,sort,is_on,picture);
            JsonRes::new(0,"success".to_string(),insert_id)
        }else
        {
            JsonRes::new(-2," category name is exist".to_string(),"")
        }

    }else{
        JsonRes::new(-1,"param error".to_string(),"")
    }
}