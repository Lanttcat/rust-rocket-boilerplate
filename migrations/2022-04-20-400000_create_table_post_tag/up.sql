-- Your SQL goes here
CREATE TABLE post_tags
(
    id         SERIAL primary key,
    content    text                     not null,
    belong       text                     not null,
    created_by uuid                     not null,
    created_at timestamp with time zone NOT NULL DEFAULT NOW()
)