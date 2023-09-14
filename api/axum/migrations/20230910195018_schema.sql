drop table if exists nodes;
CREATE TABLE IF NOT EXISTS nodes (
    id INTEGER PRIMARY KEY,
created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    ip string not null
);
