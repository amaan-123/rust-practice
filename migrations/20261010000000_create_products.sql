CREATE TABLE products (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    available_quantity INTEGER NOT NULL
        CHECK (available_quantity >= 0)
);