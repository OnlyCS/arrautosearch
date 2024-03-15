use serde_json::json;
use std::net::SocketAddr;

use actix_web::{post, App, HttpResponse, HttpServer, Responder};

const APIKEY: &str = "whatever";

#[post("/")]
async fn index(json: String) -> impl Responder {
    println!("Received: {}", json);

    // parse to serde_json::Value
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    // get value.eventType
    let event_type = value["eventType"].as_str().unwrap();

    // must be a "Grab" event
    if event_type != "Grab" {
        return HttpResponse::Ok().body("Received");
    }

    // get value.series.id
    let series_id = value["series"]["id"].as_str().unwrap();

    // new json object: { "name": "SeriesSearch", seriesId: value above }
    let search_json = json!({
        "name": "SeriesSearch",
        "seriesId": series_id
    });

    // make request to sonarr:8989/api/command?apikey=APIKEY with search_json as body for post
    let client = reqwest::Client::new();

    let res = client
        .post("http://sonarr:8989/api/command")
        .query(&[("apikey", APIKEY)])
        .header("Content-Type", "application/json")
        .body(search_json.to_string())
        .send()
        .await;

    match res {
        Ok(_) => println!("Request sent"),
        Err(e) => println!("Error: {}", e),
    }

    HttpResponse::Ok().body("Received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}
