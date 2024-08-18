#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
mod req;

#[get("/")]
async fn index() -> Template {
    let dolar_c = req::precio_compra().await.unwrap();
    let dolar_v = req::precio_venta().await.unwrap();
    Template::render("index", context!{
        dlc: dolar_c,
        dlv: dolar_v,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
