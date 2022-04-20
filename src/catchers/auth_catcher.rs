use rocket::serde::json::Json;

use crate::catchers::common_catcher::ErrorResponse;

#[catch(404)]
pub fn member_not_found() -> Json<ErrorResponse> {
    Json::from(ErrorResponse {
        message: "Member not Found".to_string(),
    })
}
