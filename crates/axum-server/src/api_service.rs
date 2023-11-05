use crate::errors::CustomError;
use db::queries::users::get_users;
use db::Pool;
use grpc_api::api::{tonic::transport::NamedService, *};
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct UsersService {
    pub pool: Pool,
}

#[tonic::async_trait]
impl grpc_api::api::users_server::Users for UsersService {
    async fn get_users(
        &self,
        _request: Request<GetUsersRequest>,
    ) -> Result<Response<GetUsersResponse>, Status> {
        // Get a client from our database pool
        let client = self
            .pool
            .get()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))
            .unwrap();

        // Get the users from the database
        let users = get_users()
            .bind(&client)
            .all()
            .await
            .map_err(|e| CustomError::Database(e.to_string()))
            .unwrap();

        // Map the structs we get from cornucopia to the structs
        // we need for our gRPC reply.
        let users = users
            .into_iter()
            .map(|user| User {
                id: user.id as u32,
                email: user.email,
            })
            .collect();

        let users = GetUsersResponse { users };

        return Ok(Response::new(users));
    }
}

impl NamedService for UsersService {
    const NAME: &'static str = "Users Service";
}
