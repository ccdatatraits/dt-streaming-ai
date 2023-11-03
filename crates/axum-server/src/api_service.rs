use crate::errors::CustomError;
// use db::queries::users;
use db::queries::users::get_users;
use db::Pool;
use grpc_api::api::*;
use tonic::{Request, Response, Status};

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
        // let users = crate::users(&client)
        //     .await
        //     .map_err(|e| CustomError::Database(e.to_string()))
        //     .unwrap();

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
// async fn load_users() {
//     let db_url = std::env::var("DATABASE_URL").unwrap();
//     let pool = create_pool(&db_url);

//     let client = pool.get().await.unwrap();

//     let users = crate::queries::users::get_users()
//         .bind(&client)
//         .all()
//         .await
//         .unwrap();

//     dbg!(users);
// }
