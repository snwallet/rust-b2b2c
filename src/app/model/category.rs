#[deny(warnings,unused_imports)]

// use mysql::*;
use mysql::prelude::*;
use crate::app::model::db;
// use crate::app::lib::common;
use serde::{Serialize, Deserialize};


#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct ShopCate{
    pub   id:u64,
    pub  cate_name:String,
    pub   parent_id:u64,
    pub   level:u8,
    pub   path:String,
    pub   sort:u16,
    pub   is_on:u8,
    pub    picture:String,
}


#[allow(dead_code)]
pub fn insert(cate_name:&str,parent_id:&str,level:&str,path:&str,sort:&str,is_on:&str,picture:&str) -> u64 {

    let params =(cate_name,parent_id,level,path,sort,is_on,picture);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "insert into shop_cate(cate_name,parent_id,level,path,sort,is_on,picture)
        values(?,?,?,?,?,?,?)".with(params).run(&mut conn).unwrap();
    ret.last_insert_id().unwrap()
}

//#[allow(dead_code)]
pub fn update(cate_name:&str,parent_id:&str,level:&str,path:&str,sort:&str,is_on:&str,picture:&str,id:&str) -> u64 {

    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update shop_cate set cate_name=?,parent_id=?,level=?,path=?,sort=?,is_on=?,picture=? where id=?"
        .with((cate_name,parent_id,level,path,sort,is_on,picture,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn allow_category_name(cate_name:&str) -> bool {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,cate_name,parent_id,level,path,sort,is_on,picture as len from shop_cate where is_delete=? and cate_name=?"
        .with((0,cate_name))
        .map(&mut conn, |(id,cate_name,parent_id,level,path,sort,is_on,picture)|{
            ShopCate{id,cate_name,parent_id,level,path,sort,is_on,picture}
        }).unwrap();
    println!("{:?}",ret);
    if ret.len()>0 { false }else{ true }
}

#[allow(dead_code)]
pub fn delete(id:&str) -> u64 {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update shop_cate set is_delete=? WHERE id=?".with((1,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn select_all() -> Vec<ShopCate> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,cate_name,parent_id,level,path,sort,is_on,picture  from shop_cate where is_delete=0"
        .map(&mut conn, |(id,cate_name,parent_id,level,path,sort,is_on,picture)|{
            ShopCate{id,cate_name,parent_id,level,path,sort,is_on,picture}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_category_name(cate_name:&str) -> Vec<ShopCate> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,cate_name,parent_id,level,path,sort,is_on,picture  from shop_cate where is_delete=? and cate_name=?"
        .with((0,cate_name))
        .map(&mut conn, |(id,cate_name,parent_id,level,path,sort,is_on,picture)|{
            ShopCate{id,cate_name,parent_id,level,path,sort,is_on,picture}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_id(id:&str) -> Vec<ShopCate> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,cate_name,parent_id,level,path,sort,is_on,picture  from shop_cate where is_delete=? and id=?"
        .with((0,id))
        .map(&mut conn, |(id,cate_name,parent_id,level,path,sort,is_on,picture)|{
            ShopCate{id,cate_name,parent_id,level,path,sort,is_on,picture}
        }).unwrap();
    println!("{:?}",ret);
    ret
}
