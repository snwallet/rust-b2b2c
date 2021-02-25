#![deny(warnings)]

mod controller;
mod model;
mod lib;

use crate::app::controller::*;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};


pub async fn run(port:u16) {
    let addr = ([127, 0, 0, 1], port).into();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(make_svc);
    println!("server run at http://{}",addr);
    if let Err(e) = server.await { eprintln!("server error: {}", e); }
}

async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/")  => file_controller::main(req).await,
        (&Method::POST, "/test") => test_controller::main(req).await,
        (&Method::POST, "/param") => param_controller::main(req).await,
        (&Method::GET, "/file")  => file_controller::main(req).await,
        //admin login
        (&Method::POST, "/admin/login") => test_controller::main(req).await,
        //store
        (&Method::POST, "/store/login") => store::login::main(req).await,
        (&Method::POST, "/store/insert") => store::insert::main(req).await,
        (&Method::POST, "/store/select") => store::select::main(req).await,
        (&Method::POST, "/store/update") => store::update::main(req).await,
        (&Method::POST, "/store/delete") => store::delete::main(req).await,
        //user curd
        (&Method::POST, "/user/insert") => test_controller::main(req).await,
        (&Method::POST, "/user/select") => test_controller::main(req).await,
        (&Method::POST, "/user/update") => test_controller::main(req).await,
        (&Method::POST, "/user/delete") => test_controller::main(req).await,
        //brand curd
        (&Method::POST, "/brand/insert") => test_controller::main(req).await,
        (&Method::POST, "/brand/select") => test_controller::main(req).await,
        (&Method::POST, "/brand/update") => test_controller::main(req).await,
        (&Method::POST, "/brand/delete") => test_controller::main(req).await,
        //category curd
        (&Method::POST, "/category/insert") => test_controller::main(req).await,
        (&Method::POST, "/category/select") => test_controller::main(req).await,
        (&Method::POST, "/category/update") => test_controller::main(req).await,
        (&Method::POST, "/category/delete") => test_controller::main(req).await,
        //goods curd
        (&Method::POST, "/goods/insert") => test_controller::main(req).await,
        (&Method::POST, "/goods/select") => test_controller::main(req).await,
        (&Method::POST, "/goods/update") => test_controller::main(req).await,
        (&Method::POST, "/goods/delete") => test_controller::main(req).await,



        _ => Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("".to_string())).unwrap()),
    }
}
