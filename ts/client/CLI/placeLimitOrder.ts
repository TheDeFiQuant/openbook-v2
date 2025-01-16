#!/usr/bin/env node
import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { OpenBookV2Client, type PlaceOrderArgs } from '../src';
import { Market } from '../../client/src/accounts/market';
import { getAssociatedTokenAddress } from '../src/utils/utils';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import fs from 'fs';
import BN from 'bn.js';

// Parse CLI arguments
const argv = yargs(hideBin(process.argv))
  .usage('Usage: $0 <command> [options]')
  .command('place-order', 'Place a limit order on OpenBook', (yargs) => {
    yargs
      .option('market', { type: 'string', demandOption: true, description: 'Market public key' })
      .option('openOrders', { type: 'string', demandOption: true, description: 'OpenOrders account public key' })
      .option('ownerKeypair', { type: 'string', demandOption: true, description: 'Path to owner keypair file' })
      .option('side', { type: 'string', choices: ['bid', 'ask'], demandOption: true, description: 'Order side (bid or ask)' })
      .option('price', { type: 'number', demandOption: true, description: 'Order price in UI units' })
      .option('size', { type: 'number', demandOption: true, description: 'Order size in UI units' });
  })
  .help()
  .alias('help', 'h').argv;

// Helper function to load a wallet keypair
function loadKeypair(filePath: string): Keypair {
  const keyData = JSON.parse(fs.readFileSync(filePath, 'utf-8'));
  return Keypair.fromSecretKey(Buffer.from(keyData));
}

// Main function to handle placing a limit order
async function main() {
  const OPENBOOK_PROGRAM_ID = new PublicKey('opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb');
  const connection = new Connection('YOUR_RPC_URL', 'confirmed');
  const owner = loadKeypair(argv.ownerKeypair as string);
  const wallet = new Wallet(owner);
  const provider = new AnchorProvider(connection, wallet, {});
  const client = new OpenBookV2Client(provider, OPENBOOK_PROGRAM_ID, {
    postSendTxCallback: ({ txid }) => {
      console.log(`Transaction submitted with ID: ${txid}`);
    },
  });

  const marketPubkey = new PublicKey(argv.market as string);
  const openOrdersPubkey = new PublicKey(argv.openOrders as string);

  try {
    console.log('Fetching and validating market data...');
    const marketDataRaw = await connection.getAccountInfo(marketPubkey);
    if (!marketDataRaw || !marketDataRaw.data) {
      throw new Error('Market data not found.');
    }
    const marketAccount = client.decodeMarket(marketDataRaw.data);

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

    const market = await Market.load(client, marketPubkey);

    console.log('Converting UI amounts to native amounts...');
    const priceNative = market.priceUiToLots(argv.price as number);
    const sizeNative = market.baseUiToLots(argv.size as number);
    const maxQuoteLotsIncludingFees = market.quoteUiToLots(argv.price * argv.size);

    console.log('Native Values:', {
      priceNative: priceNative.toString(),
      sizeNative: sizeNative.toString(),
      maxQuoteLotsIncludingFees: maxQuoteLotsIncludingFees.toString(),
    });

    console.log('Constructing place order instruction...');
    const args: PlaceOrderArgs = {
      side: argv.side === 'bid' ? { bid: {} } : { ask: {} },
      priceLots: priceNative,
      maxBaseLots: sizeNative,
      maxQuoteLotsIncludingFees,
      clientOrderId: new BN(Date.now()),
      orderType: { limit: {} },
      expiryTimestamp: new BN(0),
      selfTradeBehavior: { decrementTake: {} },
      limit: 16,
    };

    const userTokenAccount = await getAssociatedTokenAddress(
      argv.side === 'bid' ? market.account.quoteMint : market.account.baseMint,
      owner.publicKey
    );

    const [placeOrderIx] = await client.placeOrderIx(
      openOrdersPubkey,
      marketPubkey,
      market.account,
      userTokenAccount,
      args,
      []
    );

    try {
      console.log('Sending transaction...');
      const signature = await client.sendAndConfirmTransaction([placeOrderIx], {
        additionalSigners: [owner],
        txConfirmationCommitment: 'confirmed',
      });
      console.log(`Order placed successfully. Transaction Signature: ${signature}`);
    } catch (error) {
      console.error('Transaction failed:', error.message || error);
      if (error.txid) {
        console.error(`Failed Transaction ID: ${error.txid}`);
      }
      process.exit(1);
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
