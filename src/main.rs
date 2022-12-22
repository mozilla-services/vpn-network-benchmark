/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use actix_web::{
    http::Method, middleware, web, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
};
use log::info;
use std::env;

async fn index() -> HttpResponse {
    let repo_link: String = env::var("REPO_LINK")
        .expect("REPO_LINK not found")
        .parse()
        .unwrap();

    HttpResponse::Ok().body(repo_link)
}

async fn health(req: HttpRequest) -> HttpResponseBuilder {
    info!("Received health ping");

    match *req.method() {
        Method::GET => HttpResponse::Ok(),
        Method::POST => HttpResponse::Ok(),
        _ => HttpResponse::NotFound(),
    }
}

async fn upload(data: web::Bytes) -> HttpResponseBuilder {
    info!("Received upload request");

    // Only read the data and do nothing with it
    data.iter().next();

    HttpResponse::Ok()
}

fn load_env_file() {
    // Try to load vars from `.env` or fallback to `.env.local`
    match dotenv::dotenv().ok() {
        Some(_) => info!("Loading env from .env"),
        None => {
            let env_local = ".env.local";
            if dotenv::from_filename(env_local).ok().is_some() {
                info!("Loading env from {}", env_local);
            }
        }
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    load_env_file();

    let port: u16 = env::var("PORT").expect("PORT not found").parse().unwrap();

    HttpServer::new(|| {
        let max_payload_size: usize = env::var("MAX_PAYLOAD_SIZE")
            .expect("MAX_PAYLOAD_SIZE not found")
            .parse()
            .unwrap();

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::PayloadConfig::default().limit(max_payload_size))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/health").to(health))
            .service(web::resource("/upload").route(web::post().to(upload)))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[test]
    async fn test_load_env_file() -> Result<(), Error> {
        load_env_file();

        let port = env::var("PORT").ok();

        assert!(port.is_some());

        Ok(())
    }

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        load_env_file();

        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_health_get() -> Result<(), Error> {
        let app = App::new().route("/health", web::get().to(health));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_health_post() -> Result<(), Error> {
        let app = App::new().route("/health", web::post().to(health));
        let app = test::init_service(app).await;

        let req = test::TestRequest::post().uri("/health").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_upload_get() -> Result<(), Error> {
        let app = App::new().service(web::resource("/upload").route(web::post().to(upload)));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/upload").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::METHOD_NOT_ALLOWED);

        Ok(())
    }

    #[actix_web::test]
    async fn test_upload_post() -> Result<(), Error> {
        let app = App::new().route("/upload", web::post().to(upload));
        let app = test::init_service(app).await;

        let req = test::TestRequest::post().uri("/upload").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }

    #[actix_web::test]
    async fn test_upload_put() -> Result<(), Error> {
        let app = App::new().service(web::resource("/upload").route(web::post().to(upload)));
        let app = test::init_service(app).await;

        let req = test::TestRequest::put().uri("/upload").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::METHOD_NOT_ALLOWED);

        Ok(())
    }

    #[actix_web::test]
    async fn test_upload_limit() -> Result<(), Error> {
        use bytes::{BufMut, BytesMut};

        let max_payload_size: usize = 1048576; // 1 MB

        let app = App::new()
            .app_data(web::PayloadConfig::default().limit(max_payload_size))
            .service(web::resource("/upload").route(web::post().to(upload)));
        let app = test::init_service(app).await;

        let overflow_payload_size = max_payload_size + 1;
        let mut buffer = BytesMut::with_capacity(overflow_payload_size);
        while buffer.len() < overflow_payload_size {
            buffer.put_u8(0);
        }

        let req = test::TestRequest::post()
            .set_payload(buffer)
            .uri("/upload")
            .to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::PAYLOAD_TOO_LARGE);

        Ok(())
    }
}
