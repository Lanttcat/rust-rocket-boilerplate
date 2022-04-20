#[get("/healthz")]
pub fn index() -> &'static str {
    "It's OK"
}

#[cfg(test)]
mod test_healthz {
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    use crate::rocket_builder;

    #[rocket::async_test]
    async fn test() {
        let client = Client::tracked(rocket_builder()).await.unwrap();
        let r1 = client.get("/api/healthz").dispatch().await;

        assert_eq!(r1.status(), Status::Ok);
        assert_eq!(r1.into_string().await.unwrap(), "It's OK");
    }
}
