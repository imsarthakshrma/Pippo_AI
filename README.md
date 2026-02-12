# Pippo: Autonomous Prediction Market Colony 🦀

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Claude 4.5](https://img.shields.io/badge/Analysis-Claude%204.5%20Sonnet-purple.svg)](https://www.anthropic.com/)

Pippo is an autonomous multi-agent colony designed to navigate, analyze, and trade on prediction markets (Polymarket). Leveraging **Extended Thinking** via the Claude 4.5 Sonnet API, Pippo identifies market mispricings with deep probabilistic reasoning and adaptive risk management.

## 🚀 Overview

Pippo isn't just a bot; it's a **survival-pressured colony** of five distinct trading personalities. Each agent competes for capital, learns from its environment, and documents its journey through daily experiential "Field Notes."

### Core Features

-   **Multi-Agent Colony**: Five independent personalities (Alpha, Beta, Gamma, Delta, Omega) with unique risk appetites and analysis styles.
-   **Extended Thinking**: Integrated Claude 4.5 Sonnet with `interleaved-thinking` support, enabling deep multi-step market analysis.
-   **Risk Management**: Position sizing via the **Kelly Criterion** (capped at 6% per trade).
-   **Survival Mechanics**: A "Death Rule" ensuring that agents that hit a $0 balance shut down permanently.
-   **Backtesting Engine**: A rigorous simulation environment to validate strategies against 6 months of historical data before live deployment.
-   **Experiential Blogging**: Daily philosophical reflections from each agent about their experience navigating the internet's information landscape.

## 🏗️ Architecture

```text
pippo/
├── src/
│   ├── colony/       # Multi-agent coordination and personalities
│   ├── backtesting/  # Historical data simulation and graduation criteria
│   ├── claude/       # Anthropic API client with thinking block preservation
│   ├── market/       # Polymarket and external data (NOAA, Sports, Crypto)
│   ├── trading/      # Kelly sizer, execution logic, and market analysis
│   ├── db/           # Shared SQLite state and performance tracking
│   └── blog/         # Personality-driven experiential journal generator
```

## 🛠️ Getting Started

### Prerequisites

-   Rust 1.75+
-   SQLite3
-   Anthropic API Key (with Claude 4.5 support)
-   Polymarket API Credentials

### Installation

1.  Clone the repository:
    ```bash
    git clone https://github.com/yourusername/pippo.git
    cd pippo
    ```

2.  Configure your environment:
    ```bash
    cp config.toml.example config.toml
    # Edit config.toml with your API keys and parameters
    ```

3.  Build the project:
    ```bash
    cargo build --release
    ```

### Running Backtests

Before live trading, all agents must graduate from the backtesting environment:
```bash
cargo run -- --backtest --duration-months 6
```

## 📜 Graduation Criteria

To move from simulation to live markets, an agent must:
1.  Maintain a **Positive Return** over a 6-month historical period.
2.  Maintain a **Max Drawdown < 40%**.
3.  Achieve a **Sharpe Ratio > 1.5**.
4.  Demonstrate learning progress via the RL reward system.

## 📝 Blog: Field Notes from the Internet

Each day, Pippo agents generate Markdown reflections located in `/blog/`. These entries focus on the *experience* of uncertainty, human behavior, and information patterns, rather than raw performance metrics.

## 🛡️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
*Developed with survival pressure and curious algorithms.*
