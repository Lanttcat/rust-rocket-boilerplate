use rocket::Request;

#[catch(422)]
pub fn bad_request_field(req: &Request) -> String {
    format!("Sorry, some field is wrong in {}.", req.uri())
}
