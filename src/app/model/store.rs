#[deny(warnings,unused_imports)]

// use mysql::*;
use mysql::prelude::*;
use crate::app::model::db;
use crate::app::lib::common;

#[derive(Debug, PartialEq, Eq)]
struct Store {
    id: u64,
    store_title:String,
    store_name:String,
    store_pwd:String,
    store_desc:String,
    create_time:u64,
    update_time:u64
}



#[allow(dead_code)]
pub fn insert(store_title:&str, store_name:&str, store_pwd:&str, store_desc:&str) -> u64 {
    let pwd = common::md5(store_pwd);
    let time = common::get_timestamp();
    let params = (store_title,store_name,pwd,store_desc,time,time);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "insert into shop_store(store_title,store_name,store_pwd,store_desc,create_time,update_time)
        values(?,?,?,?,?,?)".with(params).run(&mut conn).unwrap();
    ret.last_insert_id().unwrap()
}

#[allow(dead_code)]
pub fn allow_store_title(store_title:&str) -> bool {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time as len from shop_store where is_delete=? and store_title=?"
        .with((0,store_title))
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    if ret.len()>0 { false }else{ true }
}

#[allow(dead_code)]
pub fn allow_store_name(store_name:&str) -> bool {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time as len from shop_store where is_delete=? and store_name=?"
        .with((0,store_name))
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    if ret.len()>0 { false }else{ true }
}