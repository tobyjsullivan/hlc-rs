extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use futures::future;
use hyper::rt::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use regex::Regex;
use std::env;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn handle_filter(req: Request<Body>) -> BoxFut {
    let response = Response::new(Body::from("FILTER"));
    Box::new(future::ok(response))
}

fn handle_group(req: Request<Body>) -> BoxFut {
    let response = Response::new(Body::from("GROUP"));
    Box::new(future::ok(response))
}

fn handle_suggest(req: Request<Body>, acct_id: u32) -> BoxFut {
    let response = Response::new(Body::from(format!("SUGGEST({})", acct_id)));
    Box::new(future::ok(response))
}

fn handle_recommend(req: Request<Body>, acct_id: u32) -> BoxFut {
    let response = Response::new(Body::from(format!("RECOMMEND({})", acct_id)));
    Box::new(future::ok(response))
}

fn handle_create_account(req: Request<Body>) -> BoxFut {
    let response = Response::new(Body::from("CREATE"));
    Box::new(future::ok(response))
}

fn handle_create_likes(req: Request<Body>) -> BoxFut {
    let response = Response::new(Body::from("ADD_LIKES"));
    Box::new(future::ok(response))
}

fn handle_update_account(req: Request<Body>, acct_id: u32) -> BoxFut {
    let response = Response::new(Body::from(format!("UPDATE({})", acct_id)));
    Box::new(future::ok(response))
}

fn handle_request(req: Request<Body>) -> BoxFut {
    lazy_static! {
        static ref reUpdatePath: Regex =
            Regex::new(r"/accounts/([a-zA-Z0-9]+)/(?:([a-zA-Z0-9]+)/)?").unwrap();
    }

    let path = req.uri().path();
    let mut part1s = None;
    let mut part2s = None;
    let mut part1id = None;
    let mut part2id = None;
    if let Some((match1, match2)) = reUpdatePath
        .captures(path)
        .map(|caps| (caps.get(1), caps.get(2)))
    {
        part1s = match1.map(|m| m.as_str());
        part2s = match2.map(|m| m.as_str());

        part1id = part1s.and_then(|s| s.parse::<u32>().ok());
        part2id = part2s.and_then(|s| s.parse::<u32>().ok());
    }

    match (req.method(), part1s, part1id, part2s, part2id) {
        (&Method::GET, Some("filter"), _, None, _) => handle_filter(req),
        (&Method::GET, Some("group"), _, _, _) => handle_group(req),
        (&Method::GET, _, Some(acct_id), Some("suggest"), _) => handle_suggest(req, acct_id),
        (&Method::GET, _, Some(acct_id), Some("recommend"), _) => handle_recommend(req, acct_id),
        (&Method::POST, Some("new"), _, _, _) => handle_create_account(req),
        (&Method::POST, Some("likes"), _, _, _) => handle_create_likes(req),
        (&Method::POST, _, Some(acct_id), None, _) => handle_update_account(req, acct_id),
        _ => {
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Box::new(future::ok(response))
        }
    }
}

fn main() {
    let port = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(_) => 8080,
    };

    // This is our socket address...
    let addr = ([127, 0, 0, 1], port).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn(handle_request)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    println!("Starting server on port {}", port);
    hyper::rt::run(server);
}
