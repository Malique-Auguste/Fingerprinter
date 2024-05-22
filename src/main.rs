pub mod user;

use user::User;

use std::convert::Infallible;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{fs, hash};

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
async fn user_id(ip: IpAddr, user_data: &str) -> RawHtml<String> {
    println!("Getting user id");
    println!("{:?}, {:?}",  ip, user_data);

    let mut user: User = serde_json::from_str(user_data).unwrap();
    user.ip = ip.to_string();

    let (id, match_value) = get_best_match(user);
    RawHtml(format!("Id: {}, likelihood: {}%", id, match_value))

}


fn get_best_match(mut user: User) -> (u64, f64) {
    let mut database_txt = {
        /*let mut temp = String::new();
        
        let mut database_file = fs::OpenOptions::new().read(true).create(true).open("database.txt").unwrap();
        database_file.read_to_string(&mut temp).unwrap();
        
        temp*/

        fs::read_to_string("database.txt").unwrap()

    };

    let mut users: Vec<User> = Vec::new();

    if database_txt != "" {
        for user_str in database_txt.split("\r\n\r\n").map(|s| s.trim()) {
            println!("{}", user_str);
            users.push(match serde_json::from_str::<User>(user_str) {
                Ok(u) => u,
                Err(e) => {
                    println!("Err: {}", e);
                    unimplemented!()
                }
            })
        }
    }

    let mut best_match_indice = 0;
    let mut best_match_value = 0.0;


    for i in 0..users.len() {
        let match_value = user.compare(&users[i]);

        if match_value > best_match_value {
            best_match_value = match_value;
            best_match_indice = i;
        }
    }

    if best_match_value >= 90.0 {
        user.id = users[best_match_indice].id
    }
    else {
        let mut hasher = DefaultHasher::new();
        user.hash(&mut hasher);

        user.id = hasher.finish();
        best_match_value = 100.0;

        users.push(user.clone());

    }

    database_txt = users.iter().map(|u| serde_json::to_string(u).unwrap()).collect::<Vec<String>>().join("\r\n\r\n");
    let mut database_file = fs::OpenOptions::new().write(true).append(false).open("database.txt").unwrap();
    database_file.write_all(database_txt.as_bytes()).unwrap();

    (user.id, best_match_value)

}