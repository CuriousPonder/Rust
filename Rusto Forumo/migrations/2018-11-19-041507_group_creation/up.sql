CREATE TABLE groups ( 
  id SERIAL PRIMARY KEY UNIQUE,
  group_ids INTEGER NOT NULL,
  group_name VARCHAR UNIQUE NOT NULL,
  subject VARCHAR NOT NULL,
  description TEXT NOT NULL
);
