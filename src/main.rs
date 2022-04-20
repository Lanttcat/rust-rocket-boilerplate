#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    rust_rocket_boilerplate::rocket_builder().launch().await;
}
