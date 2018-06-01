use std::ops::FnOnce;
use core::server::{ Request, Response };

pub struct Route <'a> {
	method: &'a str,
	path: &'a str,
	callback: fn(Request, Response) -> &'a str
}

impl <'a> Route <'a> {
	pub fn new(method: &str, path: &str, callback: fn(Request, Response) -> &'a str) -> Route<'a> {
		Route {
			method: method,
			path: path,
			callback: callback
		}
	}
}

pub struct Router <'a> {
	routes: Vec<Route<'a>>
}

impl <'a> Router <'a> {
    pub fn new() -> Router<'a> {
        Router {
            routes: vec![]
        }
    }

	pub fn add_route(&self, method: &str, path: &str, callback: fn(Request, Response) -> &'a str) {
		let r = Route {
			method: method,
			path: path,
			callback: callback
		};

		self.routes.push(r);
	}

	pub fn get(&self, path: &str, callback: fn(Request, Response) -> &'a str) {
		self.add_route("GET", path, callback);
	}
}