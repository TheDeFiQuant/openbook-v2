#!/usr/bin/env node
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { Market } from '../../client/src/accounts/market';
import { OpenBookV2Client } from '../../client/src';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

// Parse CLI arguments
const argv = yargs(hideBin(process.argv))
  .usage('Usage: $0 <command> [options]')
  .command(
    'watcher',
    'Market order book monitoring',
    (yargs) => {
      yargs
        .option('bestbidask', {
          type: 'boolean',
          description: 'Track best bid and ask prices in real-time',
        })
        .option('book', {
          type: 'boolean',
          description: 'Display order book liquidity',
        })
        .option('market', {
          type: 'string',
          demandOption: true,
          description: 'Market public key',
        });
    }
  )
  .help()
  .alias('help', 'h').argv as {
  bestbidask?: boolean;
  book?: boolean;
  market: string;
};

// Initialize Solana connection
const connection = new Connection('YOUR_RPC_URL', 'confirmed');

// Initialize provider with a stub wallet for testing
const stubWallet = new Wallet(Keypair.generate());
const provider = new AnchorProvider(connection, stubWallet, {});

// Main function
async function main() {
  const marketPubkey = new PublicKey(argv.market);

  // Load the market
  console.log(`Loading market: ${marketPubkey.toBase58()}...`);
  const client = new OpenBookV2Client(provider);
  const market = await Market.load(client, marketPubkey);

  // Real-time best bid/ask monitoring
  if (argv.bestbidask) {
    console.log('Starting best bid/ask monitoring...');
    console.log('CLI started successfully.');
    console.log('Best Bid        | Best Ask       ');
    console.log('--------------- | ---------------');

    setInterval(async () => {
      await market.loadOrderBook(); // Ensure the order book is refreshed
      const bestBid = market.bids?.best();
      const bestAsk = market.asks?.best();

      const bidPrice = bestBid?.price?.toFixed(4) || 'N/A';
      const askPrice = bestAsk?.price?.toFixed(4) || 'N/A';

      console.log(`${bidPrice.padEnd(15)} | ${askPrice.padEnd(15)}`);
    }, 1000); // Refresh every second
  }

  // Order book liquidity monitoring
  if (argv.book) {
    console.log('Starting order book liquidity monitoring...');
    console.log('CLI started successfully.');
    const depth = 10; // Top 10 levels of the order book

    setInterval(async () => {
      await market.loadOrderBook(); // Refresh the order book
      console.clear();
      console.log(
        'Price (Bid)     | Size (Bid)      | Amount (Bid)    || Price (Ask)     | Size (Ask)      | Amount (Ask)'
      );
      console.log(
        '--------------- | --------------- | --------------- || --------------- | --------------- | ---------------'
      );

      const bids = market.bids?.getL2(depth) || [];
      const asks = market.asks?.getL2(depth) || [];

      for (let i = 0; i < depth; i++) {
        const bid = bids[i] || [null, null];
        const ask = asks[i] || [null, null];

        const bidPrice = bid[0]?.toFixed(4) || 'N/A';
        const bidSize = bid[1]?.toFixed(4) || 'N/A';
        const bidAmount =
          bid[0] && bid[1] ? (bid[0] * bid[1]).toFixed(4) : 'N/A';

        const askPrice = ask[0]?.toFixed(4) || 'N/A';
        const askSize = ask[1]?.toFixed(4) || 'N/A';
        const askAmount =
          ask[0] && ask[1] ? (ask[0] * ask[1]).toFixed(4) : 'N/A';

        console.log(
          `${bidPrice.padEnd(15)} | ${bidSize.padEnd(15)} | ${bidAmount.padEnd(
            15
          )} || ${askPrice.padEnd(15)} | ${askSize.padEnd(
            15
          )} | ${askAmount.padEnd(15)}`
        );
      }
    }, 1000); // Refresh every second
  }
}

// Run the CLI
main()
  .catch((error) => {
    console.error('Error:', error);
    process.exit(1);
  });
