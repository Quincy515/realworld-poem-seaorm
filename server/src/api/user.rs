use poem_openapi::OpenApi;

pub struct UserApi;

#[OpenApi]
impl UserApi {
    #[oai(path = "/a", method = "get")]
    async fn test(&self) {}
}
