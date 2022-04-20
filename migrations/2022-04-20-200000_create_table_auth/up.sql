-- Your SQL goes here
CREATE TABLE auths
(
    member_id    uuid unique              not null,
    code         char(32),
    session_code char(36),
    created_at   timestamp with time zone NOT NULL DEFAULT NOW(),
    active_at    timestamp with time zone NOT NULL DEFAULT NOW(),
    FOREIGN KEY (member_id) REFERENCES members (id)
)
