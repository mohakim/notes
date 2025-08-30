-- Up Migration

-- Users table (password-less)
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Notes table
CREATE TABLE notes (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Magic links table (for password-less auth)
CREATE TABLE magic_links (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_notes_user_id ON notes(user_id);
CREATE INDEX idx_notes_created_at ON notes(created_at);
CREATE INDEX idx_magic_links_token ON magic_links(token);

-- Seed users
INSERT INTO users (email, display_name)
VALUES
    ('arya@winterfell.com', 'Arya Stark'),
    ('tyrion@casterlyrock.com', 'Tyrion Lannister'),
    ('daenerys@dragonstone.com', 'Daenerys Targaryen');

-- Seed notes
INSERT INTO notes (user_id, title, content) VALUES
    (1, 'Needle Practice', 'A girl has no name, but she has a sword.'),
    (2, 'Wine & Wit', 'Never forget what you are. The rest of the world will not.'),
    (3, 'Mother of Dragons', 'Dracarys!');
