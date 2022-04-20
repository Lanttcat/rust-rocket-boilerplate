use chrono::Utc;
use mailgun_v3::email::{Message, MessageBody, SendResponse};
use mailgun_v3::{Credentials, EmailAddress};
use uuid::Uuid;

use crate::config::get_config;
use crate::domains::member::repository as member_repo;
use crate::domains::session::domain::Auth;
use crate::domains::session::repository as session_repo;
use crate::helpers::db_helper::exist::exist;

pub fn code_notify(input_email: String, input_code: String) -> SendResponse {
    let msg = Message {
        to: vec![EmailAddress::address(input_email)],
        body: MessageBody::Text(format!("hello, Your login code is: {}", input_code)),
        subject: String::from("sample subject"),
        ..Default::default()
    };
    let sender = EmailAddress::address(get_config().mail_gun_sender_address);
    let creds = Credentials::new(get_config().mail_gun_api_key, get_config().mail_gun_domain);

    mailgun_v3::email::send_email(&creds, &sender, msg).unwrap()
}

pub fn create_or_update_session(id: Uuid, input_code: &String) {
    let record = Auth {
        member_id: id,
        code: Some(input_code.to_string()),
        session_code: None,
        created_at: Utc::now().naive_utc(),
        active_at: Utc::now().naive_utc(),
    };
    session_repo::insert_or_update(record);
}

pub struct SessionData {
    pub member_id: Uuid,
    pub session_code: Uuid,
}

pub fn create_session_code(input_email: &String, input_code: &String) -> Option<SessionData> {
    let member = member_repo::get_by_email(input_email.clone()).unwrap();
    let new_session_code = Uuid::new_v4();
    let used_code = input_code.to_string();

    let is_exist = exist(
        String::from("auths"),
        format!("member_id = '{}' and code = '{}'", member.id, used_code),
    );

    if !is_exist {
        return None;
    }

    session_repo::update_session_code(member.id, new_session_code);
    session_repo::update_code(member.id, None);

    Some(SessionData {
        member_id: member.id,
        session_code: new_session_code,
    })
}
