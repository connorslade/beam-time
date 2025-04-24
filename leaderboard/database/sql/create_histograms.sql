CREATE TABLE IF NOT EXISTS histograms (
    -- Level UUID
    level TEXT NOT NULL UNIQUE,

    -- Cost bins
    cost_max INTEGER NOT NULL,
    cost_01 INTEGER NOT NULL,
    cost_02 INTEGER NOT NULL,
    cost_03 INTEGER NOT NULL,
    cost_04 INTEGER NOT NULL,
    cost_05 INTEGER NOT NULL,
    cost_06 INTEGER NOT NULL,
    cost_07 INTEGER NOT NULL,
    cost_08 INTEGER NOT NULL,
    cost_09 INTEGER NOT NULL,
    cost_10 INTEGER NOT NULL,
    cost_11 INTEGER NOT NULL,
    cost_12 INTEGER NOT NULL,

    -- Latency bins
    latency_max INTEGER NOT NULL,
    latency_01 INTEGER NOT NULL,
    latency_02 INTEGER NOT NULL,
    latency_03 INTEGER NOT NULL,
    latency_04 INTEGER NOT NULL,
    latency_05 INTEGER NOT NULL,
    latency_06 INTEGER NOT NULL,
    latency_07 INTEGER NOT NULL,
    latency_08 INTEGER NOT NULL,
    latency_09 INTEGER NOT NULL,
    latency_10 INTEGER NOT NULL,
    latency_11 INTEGER NOT NULL,
    latency_12 INTEGER NOT NULL
)
