mod user;

use user::User;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use rocket::fs::FileServer;
use rocket::http::{Header, HeaderMap, Status};
use rocket::{request, Request};
use rocket::request::FromRequest;
use rocket::response::{Redirect, content::RawHtml};


#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("website"))
        .mount("/user_id", routes![user_id])
}

#[post("/<user_data>")]
async fn user_id(ip: SocketAddr, user_data: &str) {
    println!("Getting user id");
    println!("{:?}, {:?}",  ip, user_data);

    let user: User = serde_json::from_str(user_data).unwrap();

    println!("\n{:?}", user)
}