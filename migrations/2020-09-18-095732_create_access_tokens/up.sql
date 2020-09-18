CREATE TABLE access_tokens (
  id SERIAL PRIMARY KEY,
  token VARCHAR NOT NULL,
  user_id INTEGER NOT NULL,
  CONSTRAINT user_id
      FOREIGN KEY(id) 
	  REFERENCES users(id)
)