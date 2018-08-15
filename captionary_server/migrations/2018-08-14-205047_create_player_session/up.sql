CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    room_id INTEGER REFERENCES rooms(id),
    token VARCHAR(64) NOT NULL,
    ip_address VARCHAR(40) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_id ON users(id);