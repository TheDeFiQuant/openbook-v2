#!/usr/bin/env node
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client, nameToString } from '../../client/src';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

// Parse CLI arguments
const argv = yargs(hideBin(process.argv))
  .usage('Usage: $0 <command> [options]')
  .command(
    'allOOA',
    'Fetch all OpenOrders accounts and OpenOrdersIndexer for an owner',
    (yargs) => {
      yargs.option('owner', {
        type: 'string',
        demandOption: true,
        description: 'Public key of the owner',
      });
    }
  )
  .command(
    'marketOOA',
    'Fetch OpenOrders accounts and OpenOrdersIndexer for a market and owner',
    (yargs) => {
      yargs
        .option('owner', {
          type: 'string',
          demandOption: true,
          description: 'Public key of the owner',
        })
        .option('market', {
          type: 'string',
          demandOption: true,
          description: 'Public key of the market',
        });
    }
  )
  .help()
  .alias('help', 'h').argv as {
  owner?: string;
  market?: string;
  _: string[]; // Command name
};

// Initialize Solana connection
const connection = new Connection('YOUR_RPC_URL', 'confirmed');

// Initialize provider with a stub wallet for testing
const stubWallet = new Wallet(Keypair.generate());
const provider = new AnchorProvider(connection, stubWallet, {});
const client = new OpenBookV2Client(provider);

// Main function
async function main() {
  const command = argv._[0];
  if (!command) {
    console.error('Error: No command provided.');
    process.exit(1);
  }

  try {
    const ownerPk = argv.owner ? new PublicKey(argv.owner) : null;
    if (!ownerPk) {
      throw new Error('Owner public key is required.');
    }

    if (command === 'allOOA') {
      console.log(`Fetching all OpenOrders accounts for owner: ${ownerPk.toBase58()}`);

      // Fetch the OpenOrdersIndexer
      const indexer = client.findOpenOrdersIndexer(ownerPk);
      console.log(`OpenOrdersIndexer: ${indexer.toBase58()}`);

      // Fetch all OpenOrders accounts
      const openOrdersAccounts = await client.findAllOpenOrders(ownerPk);

      if (openOrdersAccounts.length > 0) {
        console.log('OpenOrders Accounts:');
        for (const [i, acc] of openOrdersAccounts.entries()) {
          try {
            const accountDetails = await client.deserializeOpenOrderAccount(acc);
            const name = accountDetails?.name ? nameToString(accountDetails.name) : 'Unnamed';
            console.log(`  ${i + 1}. ${acc.toBase58()} (Name: ${name})`);
          } catch (error) {
            console.log(`  ${i + 1}. ${acc.toBase58()} (Error fetching name: ${error.message})`);
          }
        }
      } else {
        console.log('No OpenOrders accounts found for the owner.');
      }
    } else if (command === 'marketOOA') {
      const marketPk = argv.market ? new PublicKey(argv.market) : null;
      if (!marketPk) {
        throw new Error('Market public key is required.');
      }

      console.log(`Fetching OpenOrders accounts for market: ${marketPk.toBase58()} and owner: ${ownerPk.toBase58()}`);

      // Fetch the OpenOrdersIndexer
      const indexer = client.findOpenOrdersIndexer(ownerPk);
      console.log(`OpenOrdersIndexer: ${indexer.toBase58()}`);

      // Fetch OpenOrders accounts for the market
      const openOrdersForMarket = await client.findOpenOrdersForMarket(ownerPk, marketPk);

      if (openOrdersForMarket.length > 0) {
        console.log('OpenOrders Accounts for Market:');
        for (const [i, acc] of openOrdersForMarket.entries()) {
          try {
            const accountDetails = await client.deserializeOpenOrderAccount(acc);
            const name = accountDetails?.name ? nameToString(accountDetails.name) : 'Unnamed';
            console.log(`  ${i + 1}. ${acc.toBase58()} (Name: ${name})`);
          } catch (error) {
            console.log(`  ${i + 1}. ${acc.toBase58()} (Error fetching name: ${error.message})`);
          }
        }
      } else {
        console.log(`No OpenOrders accounts found for Market ${marketPk.toBase58()} and Owner ${ownerPk.toBase58()}.`);
      }
    } else {
      console.error(`Unknown command: ${command}`);
      process.exit(1);
    }
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

// Run the CLI
main()
  .catch((error) => {
    console.error('Error:', error);
    process.exit(1);
  });
