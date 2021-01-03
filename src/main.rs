mod templates;

use templates::index::Index;

use actix_web::{
    App,
    HttpServer,
    HttpResponse,
    Result as AWResult,
    web,
};

use askama::Template;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn index() -> AWResult<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::dev::Service;
//     use actix_web::{http, test, web, App, Error};

//     #[actix_rt::test]
//     async fn test_index() -> Result<(), Error> {
//         let app = App::new().route("/", web::get().to(index));
//         let mut app = test::init_service(app).await;

//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp = app.call(req).await.unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = match resp.response().body().as_ref() {
//             Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//             _ => panic!("Response error"),
//         };

//         assert_eq!(response_body, r##"Hello world!"##);

//         Ok(())
//     }
// }