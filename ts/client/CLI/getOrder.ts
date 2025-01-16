#!/usr/bin/env node
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { OpenBookV2Client, Market } from '../../client/src';
import { OpenOrders } from '../src/accounts/openOrders';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

// Parse CLI arguments
const argv = yargs(hideBin(process.argv))
  .usage('Usage: $0 position [options]')
  .command(
    'position',
    'Fetch current position data for an OpenBook trading account',
    (yargs) => {
      yargs
        .option('wallet', {
          type: 'string',
          description: 'Wallet public key (fetch all orders)',
        })
        .option('openOrders', {
          type: 'string',
          description: 'Specific OpenOrders account public key',
        })
        .option('market', {
          type: 'string',
          description: 'Market public key (filter by market, optional)',
        })
        .check((argv) => {
          if (!argv.wallet && !argv.openOrders) {
            throw new Error('Provide either --wallet or --openOrders');
          }
          return true;
        });
    }
  )
  .help()
  .alias('help', 'h').argv as {
  wallet?: string;
  openOrders?: string;
  market?: string;
};

// Initialize Solana connection
const connection = new Connection('YOUR_RPC_URL', 'confirmed');

// Initialize provider with a stub wallet
const stubWallet = new Wallet(Keypair.generate());
const provider = new AnchorProvider(connection, stubWallet, {});

// Main function
async function main() {
  const client = new OpenBookV2Client(provider);

  try {
    if (argv.openOrders) {
      // Fetch specific OpenOrders account
      const openOrdersPubkey = new PublicKey(argv.openOrders);
      console.log(`Fetching OpenOrders account: ${openOrdersPubkey.toBase58()}`);

      const marketPubkey = argv.market ? new PublicKey(argv.market) : undefined;
      const market = marketPubkey ? await Market.load(client, marketPubkey) : undefined;

      if (market) {
        await market.loadOrderBook(); // Ensure the order book is loaded
      }

      const openOrders = await OpenOrders.load(openOrdersPubkey, market, client);

      console.log('Current Position:');
      console.log(openOrders.toPrettyString());
    } else if (argv.wallet) {
      // Fetch all OpenOrders accounts for the wallet
      const walletPubkey = new PublicKey(argv.wallet);
      console.log(`Fetching all OpenOrders accounts for wallet: ${walletPubkey.toBase58()}`);

      const marketPubkey = argv.market ? new PublicKey(argv.market) : undefined;
      const market = marketPubkey ? await Market.load(client, marketPubkey) : undefined;

      if (market) {
        await market.loadOrderBook(); // Ensure the order book is loaded
      }

      const openOrdersList = market
        ? await OpenOrders.loadNullableForMarketAndOwner(market, walletPubkey)
        : await client.findAllOpenOrders(walletPubkey);

      if (Array.isArray(openOrdersList)) {
        for (const openOrdersPubkey of openOrdersList) {
          const openOrders = await OpenOrders.load(openOrdersPubkey, market, client);
          console.log(openOrders.toPrettyString());
        }
      } else if (openOrdersList) {
        console.log(openOrdersList.toPrettyString());
      } else {
        console.log('No OpenOrders accounts found.');
      }
    }
  } catch (error) {
    console.error('Error:', error.message || error);
    process.exit(1);
  }
}

// Run the CLI
main()
  .catch((error) => {
    console.error('Unhandled Error:', error.message || error);
    process.exit(1);
  });
