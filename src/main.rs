/* This is me messing around with code from "Hands-On 
Microservices With Rust". Nothing original, except possibly
my own commentary as I understand bits */ 
use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

// So far, so very basic, just runs and serves
// text that says "Rust Microservices"
// on localhost

fn main() {
    // wee fiddle required here to get the 
    // it running in Docker, use 0.0.0.0:8000
    // instead of 127.0.0.1 - won't bind if you use that
    let addr = ([0, 0, 0, 0], 8000).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(|| {
        service_fn_ok(|_| {
            Response::new(Body::from("Rust Microservice FTW!!!!!!!!!!!!!!1111"))
        })
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
