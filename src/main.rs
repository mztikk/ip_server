use std::net::SocketAddr;

#[macro_use]
extern crate rocket;

#[get("/")]
fn get_ip(remote_addr: SocketAddr) -> String {
    format!("{}", remote_addr.ip())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_ip])
}
