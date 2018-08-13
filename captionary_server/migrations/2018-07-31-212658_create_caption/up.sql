CREATE TABLE captions (
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    points INTEGER NOT NULL DEFAULT 0,
    published_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_captions_id ON captions(id);