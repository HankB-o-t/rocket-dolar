#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
mod req;

#[get("/")]
async fn index() -> Template {
    let dolar_c = req::precio_compra(1).await.unwrap();
    let dolar_v = req::precio_venta(1).await.unwrap();
    Template::render("index", context!{
        dlc: dolar_c,
        dlv: dolar_v,
    })
}

#[get("/tarjeta")]
async fn tarjeta() -> Template {
    let dolar_c = req::precio_compra(6).await.unwrap();
    let dolar_v = req::precio_venta(6).await.unwrap();
    Template::render("tarjeta", context!{
        dlc: dolar_c,
        dlv: dolar_v,
    })
}

#[get("/oficial")]
async fn oficial() -> Template {
    let dolar_c = req::precio_compra(0).await.unwrap();
    let dolar_v = req::precio_venta(0).await.unwrap();
    Template::render("oficial", context!{
        dlc: dolar_c,
        dlv: dolar_v,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, oficial, tarjeta])
        .attach(Template::fairing())
}

// Como llegue a hacer esto:
// Se me ocurrio un dia.
//
// Como funciona:
// En el archivo main.rs (este de aca) se maneja todo el server.
// En el archivo req.rs se hacen las requests a la api del dolar.
// En la carpeta templates esta todo lo piola.

