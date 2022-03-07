use poem::web::Json;
use poem_openapi::OpenApi;
use validator::{Validate, ValidationError};

use crate::model::user::{NewUser, UserBody};

pub struct UserApi;

#[OpenApi]
impl UserApi {
    // https://realworld-docs.netlify.app/docs/specs/backend-specs/endpoints#registration
    #[oai(path = "/api/users", method = "post")]
    async fn create_user(&self, req: Json<UserBody<NewUser>>) {
        dbg!(&req);
        match req.user.validate() {
            Ok(_) => (),
            Err(_e) => {
                dbg!(ValidationError::new("参数错误"));
            }
        };
    }
}
