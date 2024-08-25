-- Create ingredients table
CREATE TABLE ingredients (
    id UUID PRIMARY KEY,
    recipe_id UUID REFERENCES recipes(id),
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    units TEXT
);
