use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

mod employee;

async fn employees(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
  let v: Vec<employee::Employee> = vec![
    employee::Employee {
      Id: String::from("100"),
      FullName: String::from("Jack Donaghy"),
      JobTitle: String::from("Writer"),
      Location: String::from("NYC"),
    },
    employee::Employee {
      Id: String::from("101"),
      FullName: String::from("Liz Lemon"),
      JobTitle: String::from("Executive"),
      Location: String::from("NYC"),
    },
  ];

  Ok(Response::new(serde_json::to_string(&v).unwrap().into()))
}

#[tokio::main]
async fn main() {
  let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

  let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(employees)) });

  let server = Server::bind(&addr).serve(make_svc);

  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }
}
