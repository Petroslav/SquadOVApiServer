ALTER TABLE share_tokens
ADD COLUMN aad BYTEA NOT NULL,
ADD COLUMN tag BYTEA NOT NULL,
ALTER COLUMN encrypted_token TYPE BYTEA USING encrypted_token::BYTEA,
ALTER COLUMN iv TYPE BYTEA USING iv::BYTEA;