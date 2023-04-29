DELETE FROM tasks WHERE account_id = 0;
DROP TABLE IF EXISTS credentials;
CREATE TABLE IF NOT EXISTS credentials (
  username text NOT NULL PRIMARY KEY,
  password text NOT NULL
);

DROP TABLE IF EXISTS tasks;
CREATE TABLE tasks (
  id text PRIMARY KEY NOT NULL,
  username integer NOT NULL,
  title text NOT NULL,
  description text NOT NULL,
  difficulty integer NOT NULL,
  created_at timestamp NOT NULL,

  FOREIGN KEY(username) REFERENCES credentials(username)
);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);

CREATE TABLE exp (
  username integer PRIMARY KEY NOT NULL,
  exp integer NOT NULL.

  FOREIGN KEY(username) REFERENCES credentials(username)
)

INSERT INTO credentials (
  account_id, user_name, password
) VALUES ( 0, 'dummy-user', '*********************' )
