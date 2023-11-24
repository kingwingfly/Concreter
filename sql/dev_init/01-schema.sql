---- Base app schema

-- User
CREATE TABLE users (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  username varchar(128) NOT NULL UNIQUE,

  -- Auth
  pwd varchar(256),
  pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
  token_salt uuid NOT NULL DEFAULT gen_random_uuid()
);

--Article
CREATE TABLE articles (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  title varchar(256) NOT NULL,
  content text NOT NULL,
  author BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Comment
CREATE TABLE comments (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1) PRIMARY KEY,

  content text NOT NULL,
  author BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  article BIGINT NOT NULL REFERENCES articles (id) ON DELETE CASCADE,
  parent_comment BIGINT REFERENCES comments (id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Entity
CREATE TABLE entities (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1) PRIMARY KEY,

  name varchar(256) NOT NULL,
  attris JSONB NOT NULL
);

-- Formula
CREATE TABLE formulas (
  id BIGINT GENERATED BY DEFAULT AS IDENTITY (START WITH 1) PRIMARY KEY,

  md TEXT NOT NULL,
  sym TEXT NOT NULL
);
