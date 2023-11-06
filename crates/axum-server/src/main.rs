mod api_service;
mod config;
mod errors;

use crate::errors::CustomError;
use axum::{
    body::Body,
    extract::Extension,
    response::{Html, Redirect},
    routing::{get, post},
    BoxError, Form, Router,
};
use grpc_api::api::{tonic::transport::Server, users_server::UsersServer};
use http::{header::CONTENT_TYPE, Request};
use serde::Deserialize;
use std::net::SocketAddr;
use tower::{make::Shared, steer::Steer, ServiceExt};

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/sign_up", post(accept_form)) // 👈 add new route
        .layer(Extension(config))
        .layer(Extension(pool.clone()))
        .map_err(BoxError::from)
        .boxed_clone();

    // Handle gRPC API requests
    let grpc = Server::builder()
        .add_service(UsersServer::new(api_service::UsersService { pool }))
        .into_service()
        .map_response(|r| r.map(axum::body::boxed))
        .boxed_clone();

    // Create a service that can respond to Web and gRPC
    let http_grpc = Steer::new(vec![app, grpc], |req: &Request<Body>, _svcs: &[_]| {
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

async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users().bind(&client).all().await?;

    // We now return HTML
    Ok(Html(ui_components::users::users(users)))
}

// 👇 create new SignUp struct
#[derive(Deserialize)]
struct SignUp {
    email: String,
}

// 👇 handle form submission
async fn accept_form(
    Extension(pool): Extension<db::Pool>,
    Form(form): Form<SignUp>,
) -> Result<Redirect, CustomError> {
    let client = pool.get().await?;

    let email = form.email;
    // TODO - accept a password and hash it
    let hashed_password = String::from("aaaa");
    let _ = db::queries::users::create_user()
        .bind(&client, &email.as_str(), &hashed_password.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/"))
}
