CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    room_id INTEGER REFERENCES rooms(id),
    username VARCHAR(32) NOT NULL,
    token TEXT NOT NULL,
    ip_address VARCHAR(40) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_id ON users(id);