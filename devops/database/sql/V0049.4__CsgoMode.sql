ALTER TABLE csgo_match_views
ADD COLUMN map VARCHAR NOT NULL,
ADD COLUMN mode VARCHAR NOT NULL,
ADD COLUMN game_server VARCHAR NOT NULL,
ADD COLUMN start_time TIMESTAMPTZ NOT NULL;