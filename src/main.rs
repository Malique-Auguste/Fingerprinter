mod user;

use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};

use rocket::fs::FileServer;
use rocket::http::{Header, HeaderMap, Status};
use rocket::{request, Request};
use rocket::request::FromRequest;
use rocket::response::{Redirect, content::RawHtml};


#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from("website"))
}
