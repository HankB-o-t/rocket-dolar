use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Dolar {
    compra: f32,
    venta: f32,
}

/*  tipos dolar con sus respectivos numeros
*   0: Dolar oficial
*   1: Dolar blue
*   2: Dolar bolsa
*   3: Dolar contado con liquidacion
*   4: Dolar mayorista
*   5: Dolar cripto
*   6: Dolar Tarjeta
* */
pub async fn precio(tdolar: usize) -> Result<Vec<String>, reqwest::Error> {
    let dolarc: Vec<Dolar> = reqwest::Client::new()
        .get("https://dolarapi.com/v1/dolares")
        .send()
        .await?
        .json()
        .await?;
    
    let dllv = dolarc[tdolar].venta.to_string();
    let dllc = dolarc[tdolar].compra.to_string();
    let dlls = vec![dllv,dllc];
    return Ok(dlls);
}


