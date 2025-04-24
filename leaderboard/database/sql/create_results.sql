CREATE TABLE IF NOT EXISTS results (
    -- User type differences between hardware ids (0) and steam ids (1) and user
    -- is some u32 bit value specific to each user
    user_type INTEGER NOT NULL,
    user INTEGER NOT NULL,

    -- Information about the request that uploaded the result
    ip INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,

    -- The textual representation of the level UUID (i know, its painful)
    level TEXT NOT NULL,
    -- The bincode (varint) encoded board (Map<Tile>)
    solution BLOB NOT NULL,
    -- The cost and latency of the solution
    cost INTEGER NOT NULL,
    latency INTEGER NOT NULL,

    UNIQUE(user_type, user, level)
);