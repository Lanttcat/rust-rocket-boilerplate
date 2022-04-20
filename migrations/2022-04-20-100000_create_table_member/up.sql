-- Your SQL goes here
CREATE TABLE members
(
    id         uuid primary key         not null,
    email      text                     not null unique,
    nickname   text                     not null unique,
    bio        text                     null,
    created_at timestamp with time zone NOT NULL DEFAULT NOW(),
    updated_at timestamp with time zone NOT NULL DEFAULT NOW()
)