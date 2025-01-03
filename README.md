<img src="https://github.com/user-attachments/assets/d88578e2-1910-4977-a8ee-e88fbf619b6f" alt="alris-modified" width="300" height="300">

# Alris - AI-Driven Yield Optimizer and Auto-Trader

Alris is an AI-powered platform designed to dynamically optimize yield and automate trading strategies on the Base layer 2 network. Built with cutting-edge technologies, Alris utilizes real-time market data and integrates with Base's infrastructure to execute informed data-driven transactions. This project is currently in its MVP (Minimum Viable Product) stage.

## MVP Demo

You can watch the demo video here: [Loom Video Link](https://www.loom.com/share/82924a7413714214b7bfd4b6f978fbe2?sid=ac8688d2-3c7c-48e0-9b87-8e14183c9659)

**Note:** The MVP is not deployed due to unresolved dependency bugs.

---

## Key Features

1. **Dynamic Yield Optimization**
   - Fetches real-time data from CoinGecko and Orca API.
   - Leverages Virtuals EVM Kit for transaction execution and updates.

2. **AI-Driven Strategy**
   - Process market data through GPTv4 to determine optimal trading and yield strategies.
   
3. **Automated Transactions**
   - Transfers funds and tracks updates directly through the Solana blockchain.

4. **Real-Time Updates**
   - Continuously monitors market conditions and adjusts strategies accordingly.

---

## Technologies Used

- **Next.js**: Framework for building the front end of the application.
- **Virtuals EVM Kit**: This is for fetching wallet data and executing on-chain transactions.
- **Switchboard Oracle**: Used to pull accurate and real-time data feeds.
- **CoinGecko API**: Fetches price data for top trading tokens.
- **Orca API**: Retrieves yield-generating pools.
- **Grok XAI**: Processes data and generates strategies using explainable AI.

---

## Getting Started

To run this project locally, follow these steps:

### Prerequisites

Ensure you have the following installed:
- Node.js (v16 or later)
- npm, yarn, pnpm, or bun (any package manager)
- Solana CLI for blockchain interaction

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/alristrade/alristrade
   cd alris
   ```

2. Install dependencies:
   ```bash
   npm install
   # or
   yarn install
   # or
   pnpm install
   # or
   bun install
   ```

3. Start the development server:
   ```bash
   npm run dev
   # or
   yarn dev
   # or
   pnpm dev
   # or
   bun dev
   ```

4. Open your browser and navigate to [http://localhost:3000](http://localhost:3000) to view the app.

---

## Learn More

To learn more about the technologies used in Alris, check out the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - Learn about the Next.js framework.
- [Virtuals Documentation](https://whitepaper.virtuals.io/) - Understand how Solana works.
- [CoinGecko API](https://www.coingecko.com/en/api) - Fetch token pricing data.
- [Orca API](https://www.orca.so/) - Get yield pool data.

---

## Known Issues

- **Dependency Bugs:** Certain unresolved dependency issues are preventing the deployment of the MVP.
- **Optimization:** Performance tuning and strategy refinement are ongoing for the AI component.

---

## Future Improvements

- Fix dependency issues to enable deployment.
- Expand support for additional DeFi protocols.
- Improve AI model accuracy for strategy generation.

---

## Contributions

Contributions, issues, and feature requests are welcome! Feel free to check out the [repository](https://github.com/maushish/alris) and submit pull requests.

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

---

