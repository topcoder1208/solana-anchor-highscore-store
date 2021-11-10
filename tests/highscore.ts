import * as assert from 'assert';
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Highscore } from '../target/types/highscore';

describe('highscore', () => {

  const provider = anchor.Provider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Highscore as Program<Highscore>;

  let game1;
  let game2;
  let game3;
  let game4;
  let game5;
  let game6;
  let game7;
  let game8;
  let game9;
  let winner;

  it('Is initialized!', async () => {
    
    // Game1 for the tests.
    game1 = anchor.web3.Keypair.generate()
    game2 = anchor.web3.Keypair.generate()
    game3 = anchor.web3.Keypair.generate()
    game4 = anchor.web3.Keypair.generate()
    game5 = anchor.web3.Keypair.generate()
    game6 = anchor.web3.Keypair.generate()
    game7 = anchor.web3.Keypair.generate()
    game8 = anchor.web3.Keypair.generate()
    game9 = anchor.web3.Keypair.generate()

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
    console.log("Your transaction signature", tx);
    let game1Account = await program.account.game1.fetch(game1.publicKey);

    assert.ok(game1Account.authority.equals(provider.wallet.publicKey))
    assert.ok(game1Account.score.toNumber() === 0)
    assert.ok(game1Account.winner.toBase58() === anchor.web3.PublicKey.default.toString())
  });

  it('Setting highscore', async () => {
    winner = anchor.web3.Keypair.generate();
    const score = new anchor.BN(3);
    await program.rpc.updateGame1(score, winner.publicKey, {
      accounts: {
        game1: game1.publicKey,
        authority: provider.wallet.publicKey,
      },
    })

    const game1Account = await program.account.game1.fetch(game1.publicKey)

    assert.ok(game1Account.authority.equals(provider.wallet.publicKey))
    assert.ok(game1Account.score.toNumber() == 3)
    assert.ok(game1Account.winner.toBase58() == winner.publicKey.toBase58())
  })

  it('Try Setting lowscore', async () => {
    const loser = anchor.web3.Keypair.generate();
    const score = new anchor.BN(2);
    await program.rpc.updateGame1(score, loser.publicKey, {
      accounts: {
        game1: game1.publicKey,
        authority: provider.wallet.publicKey,
      },
    })

    const game1Account = await program.account.game1.fetch(game1.publicKey)

    assert.ok(game1Account.authority.equals(provider.wallet.publicKey))
    assert.ok(game1Account.score.toNumber() == 3)
    assert.ok(game1Account.winner.toBase58() !== loser.publicKey.toBase58())
    assert.ok(game1Account.winner.toBase58() == winner.publicKey.toBase58())
  })

  it('Setting game 2 highscore', async () => {
    winner = anchor.web3.Keypair.generate();
    const score = new anchor.BN(3);
    await program.rpc.updateGame2(score, winner.publicKey, {
      accounts: {
        game2: game2.publicKey,
        authority: provider.wallet.publicKey,
      },
    })

    const game2Account = await program.account.game2.fetch(game2.publicKey)

    assert.ok(game2Account.authority.equals(provider.wallet.publicKey))
    assert.ok(game2Account.score.toNumber() == 3)
    assert.ok(game2Account.winner.toBase58() == winner.publicKey.toBase58())
  })

  it('Try Setting game 2 lowscore', async () => {
    const loser = anchor.web3.Keypair.generate();
    const score = new anchor.BN(2);
    await program.rpc.updateGame2(score, loser.publicKey, {
      accounts: {
        game2: game2.publicKey,
        authority: provider.wallet.publicKey,
      },
    })

    const game2Account = await program.account.game2.fetch(game2.publicKey)

    assert.ok(game2Account.authority.equals(provider.wallet.publicKey))
    assert.ok(game2Account.score.toNumber() == 3)
    assert.ok(game2Account.winner.toBase58() !== loser.publicKey.toBase58())
    assert.ok(game2Account.winner.toBase58() == winner.publicKey.toBase58())
  })
});
