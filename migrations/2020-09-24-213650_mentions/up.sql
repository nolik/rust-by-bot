CREATE TABLE mentions
(
    user_id SERIAL PRIMARY KEY,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('mentions');