use std::net::IpAddr;

#[macro_use]
extern crate rocket;

#[get("/")]
fn get_ip(ip: IpAddr) -> String {
    format!("{}", ip)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_ip])
}
