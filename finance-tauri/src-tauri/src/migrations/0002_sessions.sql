-- Migration 0002: server-side session tokens.
--
-- Replaces the previous scheme where the frontend sent the raw user id as the `user_id`
-- argument on every command (which let the WebView read any user's data by changing the
-- number). Login/register now mint an opaque, unguessable token stored here. The frontend
-- persists it and sends it back, and every authenticated command resolves it to a user id
-- server-side. Deleting a user cascades to their sessions.

CREATE TABLE IF NOT EXISTS sessions
(
    token        TEXT    PRIMARY KEY,
    user_id      INTEGER NOT NULL REFERENCES app_users (id) ON DELETE CASCADE,
    created_date TEXT    NOT NULL
);
