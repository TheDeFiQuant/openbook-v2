#!/usr/bin/env node
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client } from '../src';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

const argv = yargs(hideBin(process.argv))
  .usage('Usage: $0 --market <market> --ownerKeypair <path-to-keypair> --name <accountName>')
  .option('market', {
    type: 'string',
    demandOption: true,
    description: 'Market public key',
  })
  .option('ownerKeypair', {
    type: 'string',
    demandOption: true,
    description: 'Path to the owner keypair file',
  })
  .option('name', {
    type: 'string',
    default: 'default',
    description: 'Name for the OpenOrders account',
  })
  .help()
  .alias('help', 'h').argv as {
  market: string;
  ownerKeypair: string;
  name: string;
};

// Initialize Solana connection
const connection = new Connection('YOUR_RPC_URL', 'confirmed');

// Helper function to load a wallet keypair
function loadKeypair(filePath: string): Keypair {
  const fs = require('fs');
  const keyData = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
  return Keypair.fromSecretKey(Buffer.from(keyData));
}

// Main function to create OpenOrders account
async function main() {
  const marketPubkey = new PublicKey(argv.market);
  const owner = loadKeypair(argv.ownerKeypair);

  // Log wallet and market details
  console.log(`Using wallet: ${owner.publicKey.toBase58()}`);
  console.log(`Market: ${marketPubkey.toBase58()}`);

  // Initialize provider and client
  const wallet = new Wallet(owner);
  const provider = new AnchorProvider(connection, wallet, {});
  const client = new OpenBookV2Client(provider);

  try {
    console.log('Creating OpenOrders account...');
    const openOrdersAccountPubkey = await client.createOpenOrders(
      owner,
      marketPubkey,
      argv.name,
    );

    console.log(`OpenOrders account created successfully: ${openOrdersAccountPubkey.toBase58()}`);
  } catch (error) {
    // Handle and log errors
    console.error('Error occurred while creating OpenOrders account:');
    if (error.txid) {
      console.error(`Transaction ID: ${error.txid}`);
      console.error('Check the transaction details on Solana Explorer.');
    }
    console.error('Error details:', error.message || error);
    process.exit(1);
  }
}

// Run the CLI
main()
  .then(() => {
    console.log('CLI completed successfully.');
  })
  .catch((error) => {
    console.error('Unhandled Error:', error.message || error);
    process.exit(1);
  });
