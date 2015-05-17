extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn health_check(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "OK")))
}

fn main() {
    println!("=> Booting Web Server");
    println!("=> Iron application starting on http://localhost:3000");
    println!("=> Ctrl-C to shutdown server");

    let mut router = Router::new();
    router.get("/", health_check);
    router.get("/health-check", health_check);
    Iron::new(router).http("localhost:3000").unwrap();
}
