#[macro_use] extern crate rocket;
use rocket::Config;
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};
mod req;

// Fix for protocol
const PROTOCOL: &str = "http";

#[get("/")]
async fn index(config: &Config) -> Template {
    let dolar_c = req::precio_compra(1).await.unwrap();
    let dolar_v = req::precio_venta(1).await.unwrap();
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
    let dolar_c = req::precio_compra(6).await.unwrap();
    let dolar_v = req::precio_venta(6).await.unwrap();
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
    let dolar_c = req::precio_compra(0).await.unwrap();
    let dolar_v = req::precio_venta(0).await.unwrap();
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
