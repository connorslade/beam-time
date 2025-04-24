INSERT INTO results
VALUES (?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO
UPDATE
SET user_type = excluded.user_type,
    user = excluded.user,
    ip = excluded.ip,
    timestamp = excluded.timestamp,
    level = excluded.level,
    solution = excluded.solution,
    cost = excluded.cost,
    latency = excluded.latency;