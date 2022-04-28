use actix_web::{error, web, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
struct Flight(String, String);

async fn index() -> HttpResponse {
    HttpResponse::Ok().body(String::from("<h1>Flight Path Tracking API</h1>"))
}

async fn get_flight(data: web::Json<Vec<Flight>>) -> Result<HttpResponse, Error> {
    let flight_list = data.0;
    let len = flight_list.len();
    let mut flight_from = HashMap::new();
    let mut flight_to = HashMap::new();
    let mut airports = HashSet::new();
    for idx in 0..len {
        let flight = &flight_list[idx];
        let source = &flight.0;
        let destination = &flight.1;
        if flight_from.contains_key(destination) || flight_to.contains_key(source) {
            return Err(error::ErrorBadRequest("Invalid input"));
        }
        flight_from.insert(destination, source);
        flight_to.insert(source, destination);
        airports.insert(source);
        airports.insert(destination);
    }
    let mut flight_start = "";
    let mut flight_end = "";
    for airport in &airports {
        if !flight_from.contains_key(airport) {
            flight_start = airport;
        }
        if !flight_to.contains_key(airport) {
            flight_end = airport;
        }
    }
    // validation check
    let mut current = flight_start.to_string();
    let mut count = 1;
    loop {
        if current.eq(flight_end) {
            break;
        }
        match flight_to.get(&current) {
            None => {
                return Err(error::ErrorBadRequest("Invalid input"));
            }
            Some(&airport) => {
                current = airport.into();
            }
        }
        count = count + 1;
    }
    if count != airports.len() {
        return Err(error::ErrorBadRequest("Invalid input"));
    }
    Ok(HttpResponse::Ok().json(Flight(flight_start.to_string(), flight_end.to_string())))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/flight").route(web::post().to(get_flight)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
