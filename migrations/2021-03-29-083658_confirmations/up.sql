CREATE TABLE confirmations(
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(150) NOT NULL UNIQUE,
  expires_at TIMESTAMP NOT NULL
);