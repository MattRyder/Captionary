CREATE TABLE votes (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    caption_id INTEGER NOT NULL REFERENCES captions(id),
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uc_user_caption UNIQUE (user_id, caption_id)
);

CREATE INDEX idx_votes_id ON votes(id);