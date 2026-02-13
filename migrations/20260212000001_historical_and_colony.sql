-- Add migration script here

-- Historical Markets
CREATE TABLE IF NOT EXISTS historical_markets (
    id TEXT PRIMARY KEY,
    question TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    end_date TIMESTAMP, -- Nullable to support markets without a set end date
    outcome TEXT NOT NULL,  -- 'YES' or 'NO'
    volume REAL,
    market_type TEXT
);

-- Historical Odds snapshots
CREATE TABLE IF NOT EXISTS historical_odds (
    market_id TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    price REAL NOT NULL,  -- 0-1 probability
    PRIMARY KEY (market_id, timestamp),
    FOREIGN KEY (market_id) REFERENCES historical_markets(id)
);

-- Historical Weather
CREATE TABLE IF NOT EXISTS historical_weather (
    location TEXT NOT NULL,
    date DATE NOT NULL,
    temp_high REAL,
    temp_low REAL,
    precipitation REAL,
    conditions TEXT,
    PRIMARY KEY (location, date)
);

-- Historical Sports Results
CREATE TABLE IF NOT EXISTS historical_games (
    id TEXT PRIMARY KEY,
    sport TEXT NOT NULL,
    date DATE NOT NULL,
    home_team TEXT NOT NULL,
    away_team TEXT NOT NULL,
    home_score INTEGER,
    away_score INTEGER
);

-- Historical Crypto Prices
CREATE TABLE IF NOT EXISTS historical_crypto_prices (
    coin TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    price REAL NOT NULL,
    volume REAL,
    PRIMARY KEY (coin, timestamp)
);

-- Historical Sentiment
CREATE TABLE IF NOT EXISTS historical_sentiment (
    source TEXT NOT NULL,  -- 'reddit', 'twitter'
    topic TEXT NOT NULL,
    date DATE NOT NULL,
    sentiment_score REAL,  -- -1 to 1
    post_count INTEGER,
    PRIMARY KEY (source, topic, date)
);

-- Colony Voting
CREATE TABLE IF NOT EXISTS colony_votes (
    id TEXT PRIMARY KEY,
    market_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    trigger_reason TEXT NOT NULL,
    position TEXT NOT NULL,
    confidence REAL NOT NULL,
    reasoning_hash TEXT NOT NULL,
    edge_estimate REAL NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Capital Transfers
CREATE TABLE IF NOT EXISTS capital_transfers (
    id TEXT PRIMARY KEY,
    from_agent TEXT NOT NULL,
    to_agent TEXT NOT NULL,
    amount INTEGER NOT NULL, -- Stored in cents to avoid floating-point issues
    reason TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- RL Rewards (Enhanced)
CREATE TABLE IF NOT EXISTS rl_rewards_detailed (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    trade_id TEXT,
    reward_type TEXT NOT NULL, -- e.g., 'Outcome', 'Strategy', 'Risk', 'Milestone'
    reward_value REAL NOT NULL,
    reason TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
