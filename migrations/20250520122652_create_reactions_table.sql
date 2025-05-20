-- Add migration script here

-- Step 1: Define the enum type
CREATE TYPE reaction_type AS ENUM ('like', 'love', 'sad', 'hate', 'funny', 'skull', 'cat');

-- Step 2: Create the reactions table
CREATE TABLE reactions (
                           user_id INTEGER NOT NULL,
                           post_id INTEGER NOT NULL,
                           reaction reaction_type NOT NULL,
                           reacted_at TIMESTAMP DEFAULT NOW(),

                           PRIMARY KEY (user_id, post_id),

    -- Optional: Add foreign keys if `users` and `posts` tables exist
                           FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                           FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);