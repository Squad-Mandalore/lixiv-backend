CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    schema_title VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    data JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (schema_title) REFERENCES schemas(title) ON DELETE CASCADE,
    UNIQUE(schema_title, name)
);

CREATE INDEX idx_nodes_schema ON nodes(schema_title);
