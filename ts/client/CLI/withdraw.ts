#!/usr/bin/env node
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey, Transaction } from '@solana/web3.js';
import { OpenBookV2Client } from '../src';
import {
  getAssociatedTokenAddress,
  createAssociatedTokenAccountIdempotentInstruction,
} from '../src/utils/utils'; // Adjust the path
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import fs from 'fs';

// Parse CLI arguments
const argv = yargs(hideBin(process.argv))
  .usage('Usage: $0 withdraw [options]')
  .command('withdraw', 'Withdraw funds from OpenOrders account', (yargs) => {
    yargs
      .option('market', { type: 'string', demandOption: true, description: 'Market public key' })
      .option('openOrders', { type: 'string', demandOption: true, description: 'OpenOrders account public key' })
      .option('ownerKeypair', { type: 'string', demandOption: true, description: 'Path to owner keypair file' });
  })
  .help()
  .alias('help', 'h').argv;

// Helper function to load a wallet keypair
function loadKeypair(filePath: string): Keypair {
  const keyData = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
  return Keypair.fromSecretKey(Buffer.from(keyData));
}

// Helper function to validate and fetch market data
async function validateAndFetchMarket(
  connection: Connection,
  client: OpenBookV2Client,
  marketPubkey: PublicKey
): Promise<any> {
  const marketDataRaw = await connection.getAccountInfo(marketPubkey);
  if (!marketDataRaw || !marketDataRaw.data) {
    throw new Error('Market data not found.');
  }
  console.log('Market data fetched successfully.');
  return client.decodeMarket(marketDataRaw.data);
}

// Helper function to ensure ATAs exist
async function ensureAssociatedTokenAccount(
  connection: Connection,
  owner: Keypair,
  mint: PublicKey,
  walletPublicKey: PublicKey
): Promise<PublicKey> {
  console.log(`Ensuring associated token account for mint: ${mint.toBase58()}`);
  const ata = await getAssociatedTokenAddress(mint, walletPublicKey, true); // Allow off-curve owners
  const ataExists = await connection.getAccountInfo(ata);
  if (!ataExists) {
    console.log(`Creating associated token account for mint: ${mint.toBase58()}`);
    const createATAIx = await createAssociatedTokenAccountIdempotentInstruction(
      owner.publicKey,
      walletPublicKey,
      mint
    );
    const transaction = new Transaction().add(createATAIx);
    const signature = await connection.sendTransaction(transaction, [owner], { skipPreflight: false });
    console.log(`ATA creation transaction signature: ${signature}`);
  } else {
    console.log(`ATA already exists for mint: ${mint.toBase58()}`);
  }
  return ata;
}

// Main function to handle CLI commands
async function main() {
  const command = argv._[0];
  if (!command) {
    console.error('Error: No command provided.');
    process.exit(1);
  }

  const connection = new Connection('YOUR_RPC_URL', 'confirmed');
  const owner = loadKeypair(argv.ownerKeypair as string);
  const wallet = new Wallet(owner);
  const provider = new AnchorProvider(connection, wallet, {});
  const client = new OpenBookV2Client(provider);

  const marketPubkey = new PublicKey(argv.market as string);
  const openOrdersPubkey = new PublicKey(argv.openOrders as string);

  try {
    console.log('Fetching and validating market data...');
    const marketAccount = await validateAndFetchMarket(connection, client, marketPubkey);

    console.log('Market Account:', {
      baseDecimals: marketAccount.baseDecimals,
      quoteDecimals: marketAccount.quoteDecimals,
    });

    console.log('Deserializing OpenOrders account...');
    const openOrdersAccount = await client.deserializeOpenOrderAccount(openOrdersPubkey);
    if (!openOrdersAccount) {
      throw new Error('OpenOrders account not found.');
    }
    console.log('OpenOrders account deserialized successfully.');

    if (openOrdersAccount.market.toString() !== marketPubkey.toString()) {
      throw new Error('OpenOrders account does not belong to the specified market.');
    }
    console.log('OpenOrders account validated against market.');

    console.log('Ensuring associated token accounts...');
    const baseTokenAccount = await ensureAssociatedTokenAccount(
      connection,
      owner,
      marketAccount.baseMint,
      wallet.publicKey
    );
    const quoteTokenAccount = await ensureAssociatedTokenAccount(
      connection,
      owner,
      marketAccount.quoteMint,
      wallet.publicKey
    );

    console.log('Preparing withdrawal...');
    const [withdrawIx, signers] = await client.settleFundsIx(
      openOrdersPubkey,
      openOrdersAccount,
      marketPubkey,
      marketAccount,
      baseTokenAccount,
      quoteTokenAccount,
      null, // No referrer account
      wallet.publicKey // Penalty payer is the wallet owner
    );

    console.log('Sending transaction...');
    const transaction = new Transaction().add(withdrawIx);
    const signature = await connection.sendTransaction(transaction, [owner, ...signers], { skipPreflight: false });
    console.log(`Withdrawal transaction sent successfully. Signature: ${signature}`);
  } catch (error) {
    console.error('Error:', error.message || error);
    console.trace(error);
    process.exit(1);
  }
}

// Run CLI
main()
  .catch((error) => {
    console.error('Unhandled Error:', error.message || error);
    console.trace(error);
    process.exit(1);
  });
