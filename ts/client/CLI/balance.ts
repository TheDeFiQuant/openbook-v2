#!/usr/bin/env node
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { OpenBookV2Client, Market } from '../../client/src';
import { OpenOrders } from '../src/accounts/openOrders';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

// Parse CLI arguments
const argv = yargs(hideBin(process.argv))
  .usage('Usage: $0 <command> [options]')
  .command(
    'balance',
    'Fetch balances from OpenBook trading account',
    (yargs) => {
      yargs
        .option('openOrders', {
          type: 'string',
          demandOption: true,
          description: 'OpenOrders account public key',
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
  openOrders: string;
  market: string;
};

// Initialize Solana connection
const connection = new Connection('YOUR_RPC_URL', 'confirmed');

// Initialize provider with a stub wallet for testing
const stubWallet = new Wallet(Keypair.generate());
const provider = new AnchorProvider(connection, stubWallet, {});

// Main function
async function main() {
  const openOrdersPubkey = new PublicKey(argv.openOrders);
  const marketPubkey = new PublicKey(argv.market);

  console.log(`Fetching balances for OpenOrders account: ${openOrdersPubkey.toBase58()}`);
  try {
    // Load the market
    console.log(`Loading market: ${marketPubkey.toBase58()}...`);
    const client = new OpenBookV2Client(provider);
    const market = await Market.load(client, marketPubkey);

    // Load OpenOrders and fetch UI balances
    const openOrders = await OpenOrders.load(openOrdersPubkey, market, client);
    const baseBalanceUi = openOrders.getBaseBalanceUi();
    const quoteBalanceUi = openOrders.getQuoteBalanceUi();

    console.log('Balances:');
    console.log(`  Base Token Balance: ${baseBalanceUi}`);
    console.log(`  Quote Token Balance: ${quoteBalanceUi}`);
  } catch (error) {
    console.error('Error:', error.message || error);
    process.exit(1);
  }
}

// Run the CLI
main()
  .catch((error) => {
    console.error('Unhandled Error:', error);
    process.exit(1);
  });
