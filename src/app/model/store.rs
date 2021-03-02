#[deny(warnings,unused_imports)]

// use mysql::*;
use mysql::prelude::*;
use crate::app::model::db;
use crate::app::lib::common;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq,Serialize, Deserialize,)]
pub struct Store {
    pub id: u64,
    pub store_title:String,
    pub store_name:String,
    pub store_pwd:String,
    pub store_desc:String,
    pub create_time:u64,
    pub update_time:u64
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
pub fn update(store_title:&str, store_name:&str,store_desc:&str, id:&str) -> u64 {
    let time = common::get_timestamp();
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update shop_store set store_title=?,store_name=?,store_desc=?,update_time=? where id=?"
        .with((store_title,store_name,store_desc,time,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn allow_store_title(store_title:&str) -> bool {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time from shop_store where is_delete=? and store_title=?"
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
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time from shop_store where is_delete=? and store_name=?"
        .with((0,store_name))
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    if ret.len()>0 { false }else{ true }
}

#[allow(dead_code)]
pub fn delete(id:&str) -> u64 {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update shop_store set is_delete=? WHERE id=?".with((1,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}

#[allow(dead_code)]
pub fn select_all() -> Vec<Store> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time  from shop_store where is_delete=0"
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_store_title(store_title:&str) -> Vec<Store> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time  from shop_store where is_delete=? and store_title=?"
        .with((0,store_title))
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn select_id(id:&str) -> Vec<Store> {
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time  from shop_store where is_delete=? and id=?"
        .with((0,id))
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn login(name:&str,pwd:&str) -> Vec<Store> {
    let real_pwd = common::md5(pwd);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time  from shop_store where is_delete=? and store_name=? and store_pwd=?"
        .with((0,name,real_pwd))
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn auth_id_pwd(pwd:&str,id:&str) -> Vec<Store> {
    let real_pwd = common::md5(pwd);
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "select id,store_title,store_name,store_pwd,store_desc,create_time,update_time  from shop_store where is_delete=? and id=? and store_pwd=?"
        .with((0,id,real_pwd))
        .map(&mut conn, |(id,store_title,store_name,store_pwd,store_desc,create_time,update_time)|{
            Store{id,store_title,store_name,store_pwd,store_desc,create_time,update_time}
        }).unwrap();
    println!("{:?}",ret);
    ret
}

#[allow(dead_code)]
pub fn change_pwd(pwd:&str, id:&str) -> u64 {
    let real_pwd = common::md5(pwd);
    let time = common::get_timestamp();
    let mut conn = db::pool().get_conn().unwrap();
    let ret = "update shop_store set store_pwd=?,update_time=? where id=?"
        .with((real_pwd,time,id)).run(&mut conn).unwrap();
    println!("ret:{:?}",ret.affected_rows());
    ret.affected_rows()
}