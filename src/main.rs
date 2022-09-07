/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use actix_web::{
    App,
    http::Method,
    HttpRequest,
    HttpResponse,
    HttpResponseBuilder,
    HttpServer,
    middleware,
    web
};
use futures::stream::StreamExt;
use log::{error, info};

async fn health(req: HttpRequest) -> HttpResponseBuilder {
    info!("Received health ping");

    match *req.method() {
        Method::GET => HttpResponse::Ok(),
        Method::POST => HttpResponse::Ok(),
        _ => HttpResponse::NotFound(),
    }
}

async fn upload(req: HttpRequest, mut body: web::Payload) -> HttpResponseBuilder {
    info!("Received upload request");

    match *req.method() {
        Method::GET => HttpResponse::MethodNotAllowed(),
        Method::POST => {
            info!("Start upload");

            // Only read the data and do not need to do anything with them
            while let Some(data) = body.next().await {
                match data {
                    Ok(_bytes) => (),
                    Err(error) => {
                        error!("An error occurred: {:?}", error);
                        return HttpResponse::BadRequest();
                    },
                };
            
            }

            return HttpResponse::Ok();
        },
        _ => HttpResponse::NotFound(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/health").to(health))
            .service(web::resource("/upload").to(upload))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};
    use super::*;

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
        let app = App::new().route("/upload", web::get().to(upload));
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
        let app = App::new().route("/upload", web::put().to(upload));
        let app = test::init_service(app).await;

        let req = test::TestRequest::put().uri("/upload").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);

        Ok(())
    }
}
