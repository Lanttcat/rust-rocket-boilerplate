-- Your SQL goes here
CREATE TABLE posts
(
    id                  uuid PRIMARY KEY         NOT NULL,
    title               TEXT                     NOT NULL,
    description         TEXT                     NOT NULL,
    votes_received      UUID[]                   NOT NULL,
    anti_votes_received UUID[]                   NOT NULL,
    tags                INTEGER[]                NOT NULL,
    created_by          UUID                     NOT NULL,
    created_at          TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (created_by) REFERENCES members (id)
)