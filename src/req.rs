use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Dolar {
    compra: f32,
    venta: f32,
}

pub async fn precio_venta() -> Result<String, reqwest::Error> {
    let dolarc: Vec<Dolar> = reqwest::Client::new()
        .get("https://dolarapi.com/v1/dolares")
        .send()
        .await?
        .json()
        .await?;
    
    let dllv = dolarc[1].venta.to_string();
    return Ok(dllv);
}

pub async fn precio_compra() -> Result<String, reqwest::Error> {
    let dolarc: Vec<Dolar> = reqwest::Client::new()
        .get("https://dolarapi.com/v1/dolares")
        .send()
        .await?
        .json()
        .await?;
    
    let dllc = dolarc[1].compra.to_string();
    return Ok(dllc);
}

/*
*
*   Como funciona esta parte de la app?
*   Primero, se hace el request a la api del dolar y luego se
*   manejan los datos devueltos con la estructura "Dolar".
*   Finalmente, estos datos son retornados para usarse.
*
* */
