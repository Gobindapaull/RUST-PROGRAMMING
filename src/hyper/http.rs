use std::{convert::Infallible, net::SocketAddr};
use hyper::{service::{make_service_fn, service_fn}, Body, Request, Response, Server};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, Infallible>(service_fn(handle_request))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    println!("server is running on {:?}", addr);
    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello world")))
}
