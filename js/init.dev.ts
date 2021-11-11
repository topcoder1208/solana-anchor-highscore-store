const { clusterApiUrl, Connection, PublicKey, Keypair } = require("@solana/web3.js");
const { TOKEN_PROGRAM_ID, Token } = require("@solana/spl-token");
const anchor = require('@project-serum/anchor');

const fs = require('fs');

const path = require('path');
const os = require("os");
const homedir = '/home/wstar';

const idl = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../target/idl/highscore.json')));
const programID = new PublicKey(idl.metadata.address);

const walletKeyData = JSON.parse(fs.readFileSync(homedir + '/.config/solana/id.json'));
const walletKeypair = Keypair.fromSecretKey(new Uint8Array(walletKeyData));
const wallet = new anchor.Wallet(walletKeypair);

const connection = new Connection(clusterApiUrl('devnet'))

function getProvider() {
    const provider = new anchor.Provider(
        connection, wallet, { preflightCommitment: "processed" },
    );
    return provider;
}

const provider = getProvider();
let program = new anchor.Program(idl, programID, provider);
async function initialize() {

	const game1RawData = fs.readFileSync(path.resolve(__dirname, '../games/game1.json'));
	const game1KeyData = JSON.parse(game1RawData);
	const game1 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game1KeyData));

	const game2RawData = fs.readFileSync(path.resolve(__dirname, '../games/game2.json'));
	const game2KeyData = JSON.parse(game2RawData);
	const game2 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game2KeyData));

	const game3RawData = fs.readFileSync(path.resolve(__dirname, '../games/game3.json'));
	const game3KeyData = JSON.parse(game3RawData);
	const game3 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game3KeyData));

	const game4RawData = fs.readFileSync(path.resolve(__dirname, '../games/game4.json'));
	const game4KeyData = JSON.parse(game4RawData);
	const game4 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game4KeyData));

	const game5RawData = fs.readFileSync(path.resolve(__dirname, '../games/game5.json'));
	const game5KeyData = JSON.parse(game5RawData);
	const game5 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game5KeyData));

	const game6RawData = fs.readFileSync(path.resolve(__dirname, '../games/game6.json'));
	const game6KeyData = JSON.parse(game6RawData);
	const game6 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game6KeyData));

	const game7RawData = fs.readFileSync(path.resolve(__dirname, '../games/game7.json'));
	const game7KeyData = JSON.parse(game7RawData);
	const game7 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game7KeyData));

	const game8RawData = fs.readFileSync(path.resolve(__dirname, '../games/game8.json'));
	const game8KeyData = JSON.parse(game8RawData);
	const game8 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game8KeyData));

	const game9RawData = fs.readFileSync(path.resolve(__dirname, '../games/game9.json'));
	const game9KeyData = JSON.parse(game9RawData);
	const game9 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(game9KeyData));

    const tx = await program.rpc.initialize(provider.wallet.publicKey, {
      accounts: {
        game1: game1.publicKey,
        game2: game2.publicKey,
        game3: game3.publicKey,
        game4: game4.publicKey,
        game5: game5.publicKey,
        game6: game6.publicKey,
        game7: game7.publicKey,
        game8: game8.publicKey,
        game9: game9.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [game1,game2,game3,game4,game5,game6,game7,game8,game9],
    });
}

initialize();
