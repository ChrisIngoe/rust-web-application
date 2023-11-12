use std::{env, io};
use log::{info};

use actix_files::Files;
use actix_web::{web::Data, get, App, HttpServer, HttpResponse, Responder};
use handlebars::Handlebars;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
struct CustomText {
    datetime: String,
    text: &'static str,
}

#[get("/")]
async fn compliment(hb: Data<Handlebars<'_>>) -> impl Responder {
    let now: DateTime<Utc> = Utc::now();
    info!("Info message: {}", now);
    let custom_text = CustomText {
        datetime: now.to_string(),
        text: "Your message has been received, thank you!",
    };
    let html = hb.render("template", &custom_text).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    let address = env::var("BIND_ADDRESS")
        .unwrap_or_else(|_err| "localhost:8080".to_string());

    let template_service = {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_templates_directory(".html", "web/templates")
            .unwrap();

        Data::new(handlebars)
    };

    let server = move || App::new()
        .app_data(template_service.clone())
        .service(Files::new("/public", "web/public").show_files_listing())
        .service(compliment);

    HttpServer::new(server)
        .bind(address)?
        .run()
        .await
}