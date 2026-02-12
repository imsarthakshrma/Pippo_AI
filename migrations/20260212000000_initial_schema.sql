-- Create trades table
CREATE TABLE IF NOT EXISTS trades (
    id TEXT PRIMARY KEY NOT NULL,
    market_id TEXT NOT NULL,
    outcome TEXT NOT NULL,
    amount_usd REAL NOT NULL,
    price REAL NOT NULL,
    status TEXT NOT NULL, -- "pending", "executed", "settled"
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create market snapshots table
CREATE TABLE IF NOT EXISTS market_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    market_id TEXT NOT NULL,
    odds REAL NOT NULL,
    volume_24h REAL NOT NULL,
    scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create thinking logs table
CREATE TABLE IF NOT EXISTS thinking_logs (
    id TEXT PRIMARY KEY NOT NULL,
    trade_id TEXT,
    raw_thinking TEXT NOT NULL,
    prompt_tokens INTEGER NOT NULL,
    thinking_tokens INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (trade_id) REFERENCES trades(id)
);

-- Create RL rewards table
CREATE TABLE IF NOT EXISTS rl_rewards (
    id TEXT PRIMARY KEY NOT NULL,
    milestone TEXT NOT NULL,
    reward_value REAL NOT NULL,
    achieved_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);
