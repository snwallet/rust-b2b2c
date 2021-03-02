#[deny(warnings,unused_imports)]

// use mysql::*;
use mysql::prelude::*;
use crate::app::model::db;
use crate::app::lib::common;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq,Serialize, Deserialize,)]
pub struct Brand {
    pub id: u64,
    pub brand_name:String,
    pub brand_logo:String,
    pub brand_url:String,
    pub sort:u64,
    pub create_time:u64,
    pub update_time:u64
}

#[allow(dead_code)]
pub fn insert(brand_name:&str,brand_logo:&str,brand_url:&str,sort:u64) -> u64 {
    let time = common::get_timestamp();
    let params = (brand_name,brand_logo,brand_url,sort,time,time);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "insert into shop_brand(brand_name,brand_logo,brand_url,sort,create_time,update_time)
        values(?,?,?,?,?,?)".with(params).run(&mut conn).unwrap();
    ret.last_insert_id().unwrap()
}

#[allow(dead_code)]
pub fn allow_brand_name(brand_name:&str) -> bool {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,brand_name,brand_logo,brand_url,sort,create_time,update_time from shop_brand where is_delete=? and brand_name=?"
        .with((0,brand_name))
        .map(&mut conn, |(id,brand_name,brand_logo,brand_url,sort,create_time,update_time)|{
            Brand{id,brand_name,brand_logo,brand_url,sort,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    if ret.len()>0 { false }else{ true }
}

#[allow(dead_code)]
pub fn update(brand_name:&str,brand_logo:&str,brand_url:&str,sort:u64,id:u64) -> u64 {
    let time = common::get_timestamp();
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update shop_brand set brand_name=?,brand_logo=?,brand_url=?,brand_sort,update_time=? where id=?"
        .with((brand_name,brand_logo,brand_url,sort,time,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn delete(id:&str) -> u64 {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update shop_brand set is_delete=? WHERE id=?".with((1,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}