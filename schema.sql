DELETE FROM tasks WHERE account_id = 0;
DROP TABLE IF EXISTS credentials;
CREATE TABLE IF NOT EXISTS credentials (
  account_id integer PRIMARY KEY AUTOINCREMENT,
  user_name text NOT NULL,
  password text NOT NULL
);

DROP TABLE IF EXISTS tasks;
CREATE TABLE tasks (
  id text PRIMARY KEY NOT NULL,
  account_id integer NOT NULL,
  title text NOT NULL,
  description text NOT NULL,
  difficulty integer NOT NULL,
  created_at timestamp NOT NULL,

  FOREIGN KEY(account_id) REFERENCES credentials(account_id)
);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);

INSERT INTO credentials (
  account_id, user_name, password
) VALUES ( 0, 'dummy-user', '*********************' )
