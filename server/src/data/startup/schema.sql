ALTER ROLE postgres WITH PASSWORD 'mysecretpassword';

-- Add Types
CREATE TYPE game_phase AS ENUM (
  'SpringMovement',
  'SpringRetreat',
  'FallMovement',
  'FallRetreat',
  'WinterBuild'
);
CREATE TYPE player_status AS ENUM ('active', 'eliminated', 'won', 'spectator');
CREATE TYPE nation AS ENUM ('England', 'France', 'Germany', 'Italy', 'Austria', 'Russia', 'Turkey');

-- Add Tables
CREATE TABLE users (
  user_id SERIAL PRIMARY KEY,
  username VARCHAR(255),
  password_hash TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE games (
  game_id SERIAL PRIMARY KEY,
  name VARCHAR(255),
  year integer NOT NULL,
  game_phase game_phase NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT now()
);


CREATE TABLE players_in_game (
    player_in_game_id SERIAL PRIMARY KEY,
    game_id INTEGER NOT NULL REFERENCES games(game_id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    nation nation NOT NULL,
    status player_status NOT NULL DEFAULT 'active',
    joined_at TIMESTAMP NOT NULL DEFAULT now(),
    UNIQUE (game_id, user_id),
    UNIQUE (game_id, nation)
);

-- Fake Users
INSERT INTO users (username, password_hash) VALUES
  ('alice', 'HASH_PLACEHOLDER'),
  ('bob',   'HASH_PLACEHOLDER'),
  ('carol', 'HASH_PLACEHOLDER'),
  ('dan',   'HASH_PLACEHOLDER'),
  ('eve',   'HASH_PLACEHOLDER'),
  ('frank', 'HASH_PLACEHOLDER'),
  ('grace', 'HASH_PLACEHOLDER');

-- Game
INSERT INTO games (name, year, game_phase) VALUES
  ('Europe 1901', 1901, 'SpringMovement');

-- Easiest way alice -> grace 
INSERT INTO players_in_game (game_id, user_id, nation)
VALUES
  (1, 1, 'England'), 
  (1, 2, 'France'),
  (1, 3, 'Germany'),
  (1, 4, 'Italy'),
  (1, 5, 'Austria'),
  (1, 6, 'Russia'),
  (1, 7, 'Turkey');