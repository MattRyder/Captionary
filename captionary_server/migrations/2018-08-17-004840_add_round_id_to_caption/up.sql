ALTER TABLE captions ADD round_id INT NOT NULL DEFAULT 0;
ALTER TABLE captions ADD CONSTRAINT fk_round_id FOREIGN KEY (round_id) REFERENCES rounds(id);