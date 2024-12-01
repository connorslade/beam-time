CREATE TABLE IF NOT EXISTS results (
    user INTEGER NOT NULL,
    ip INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,

    level BLOB NOT NULL,
    cost INTEGER NOT NULL,
    latency INTEGER NOT NULL,

    solution BLOB NOT NULL
);