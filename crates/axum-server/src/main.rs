mod api_service;
mod config;
mod errors;

use crate::errors::CustomError;
use axum::{body::Body, extract::Extension, response::Json, routing::get, Router};
use db::User;
// use grpc_api::api::api_server::UsersServer;
// use grpc_api::api::tonic;
// use grpc_api::api::tonic::transport::Server;
use http::{header::CONTENT_TYPE, Request};
use std::net::SocketAddr;
use tower::{make::Shared, steer::Steer, ServiceExt};

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .layer(Extension(config))
        .layer(Extension(pool.clone()))
        .boxed_clone();

    // Handle gRPC API requests
    // let grpc = Server::builder()
    //     .add_service(TraceServer::new(api::trace_grpc_service::TraceService {
    //         pool,
    //     }))
    //     .into_service()
    //     .map_response(|r| r.map(axum::body::boxed))
    //     .boxed_clone();

    // Create a service that can respond to Web and gRPC
    let http_grpc = Steer::new(vec![app /*, grpc*/], |req: &Request<Body>, _svcs: &[_]| {
        if req.headers().get(CONTENT_TYPE).map(|v| v.as_bytes()) != Some(b"application/grpc") {
            0
        } else {
            1
        }
    });

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(Shared::new(http_grpc))
        .await
        .unwrap();
}

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Json<Vec<User>>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    Ok(Json(users))
}
