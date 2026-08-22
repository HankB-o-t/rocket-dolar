#[macro_use] extern crate rocket;
use rocket::Config;
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};
mod req;

// Fix for protocol
const PROTOCOL: &str = "http";

#[get("/")]
async fn index(config: &Config) -> Template {
    let dolar = req::precio(1).await.unwrap();
    let dolar_c = &dolar[0];
    let dolar_v = &dolar[1];
    Template::render("index", context!{
        dlc: dolar_c,
        dlv: dolar_v,
        address: config.address.to_string(),
        port: config.port,
        protocol: PROTOCOL.to_string(),
    })
}

#[get("/tarjeta")]
async fn tarjeta(config: &Config) -> Template {
    let dolar = req::precio(6).await.unwrap();
    let dolar_c = &dolar[0];
    let dolar_v = &dolar[1];
    Template::render("tarjeta", context!{
        dlc: dolar_c,
        dlv: dolar_v,
        address: config.address.to_string(),
        port: config.port,
        protocol: PROTOCOL.to_string(),
    })
}

#[get("/oficial")]
async fn oficial(config: &Config) -> Template {
    let dolar = req::precio(0).await.unwrap();
    let dolar_c = &dolar[0];
    let dolar_v = &dolar[1];
    Template::render("oficial", context!{
        dlc: dolar_c,
        dlv: dolar_v,
        address: config.address.to_string(),
        port: config.port,
        protocol: PROTOCOL.to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, oficial, tarjeta])
        .mount("/static", FileServer::from("static/"))
        .attach(Template::fairing())
}
