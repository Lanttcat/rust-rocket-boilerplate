table! {
    members (id) {
        id -> Uuid,
        email -> Text,
        nickname -> Text,
        bio -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }

}

table! {
    auths (member_id) {
        member_id -> Uuid,
        code -> Nullable<Text>,
        session_code -> Nullable<Text>,
        created_at -> Timestamp,
        active_at -> Timestamp,
    }
}

table! {
    post_tags (id) {
        id -> Integer,
        belong -> Text,
        content -> Text,
        created_by -> Uuid,
        created_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        votes_received -> Array<Uuid>,
        anti_votes_received -> Array<Uuid>,
        tags -> Array<Integer>,
        created_by -> Uuid,
        created_at -> Timestamp,
    }
}
