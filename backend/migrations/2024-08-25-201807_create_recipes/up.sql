-- Create recipes table
CREATE TABLE recipes (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    preparation_time INTEGER,
    cooking_time INTEGER,
    servings INTEGER,
    tags TEXT[],
    categories UUID[]
);
