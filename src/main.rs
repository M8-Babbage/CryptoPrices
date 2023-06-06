// Importaciones en Rust
use serde::{Deserialize, Serialize};
use std::io::stdin;

// Función principal para arrancar el programa
fn main() {
    // Variable mutable, todas son inmutables por defecto
    let mut coin: String = String::new();
    // Mostrar mensaje en consola
    println!("Ingrese el nombre de la moneda: ");
    // Entrada de datos por consola, se guarda en la variable coin aplicando borrowing y el expect es para manejar el error
    stdin()
        .read_line(&mut coin)
        .expect("Ocurrió un error al obtener el input");

    let result_price = get_price(&coin);
    match result_price {
        Ok(price) => println!("El precio es de ${price} USD"),
        Err(error) => println!("{error} occurred"),
    }
}

// Recibe una referencia del String coin, y se retorna un String
fn get_price(coin: &str) -> Result<String, ureq::Error> {
    // Request para obtener el precio real de la moneda
    let body: String = ureq::get(&format!(
        "https://api.coingecko.com/api/v3/coins/{}?localization=false",
        coin
    ))
    .call()?
    .into_string()?;
    let coin_data: CoinData = serde_json::from_str(&body).unwrap();
    Ok(coin_data.market_data.current_price.usd.to_string())
}

// Se activa serde para poder serializar y deserializar los datos "JSON, revisar cargo.toml", las struts son como interfaces en Java
#[derive(Serialize, Deserialize)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData,
}

#[derive(Serialize, Deserialize)]
struct MarketData {
    current_price: Prices,
}

#[derive(Serialize, Deserialize)]
struct Prices {
    usd: f32,
}
