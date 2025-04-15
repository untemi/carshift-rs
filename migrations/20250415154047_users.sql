CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL,
  passhash TEXT NOT NULL,
  firstname TEXT NOT NULL,
  lastname TEXT,
  pfp_file TEXT,
  phone TEXT,
  email TEXT
);

CREATE UNIQUE INDEX idx_users_username ON users (username);
