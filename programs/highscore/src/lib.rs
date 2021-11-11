use anchor_lang::prelude::*;

declare_id!("5iZs1BnZ2BK1A51EavZZaPyhgXS48kqqjpnhTDsebr8R");

#[program]
pub mod highscore {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> ProgramResult {

        let game1 = &mut ctx.accounts.game1;
        game1.authority = authority;
        game1.score = 0;
        game1.winner = Pubkey::default();

        let game2 = &mut ctx.accounts.game2;
        game2.authority = authority;
        game2.score = 0;
        game2.winner = Pubkey::default();

        let game3 = &mut ctx.accounts.game3;
        game3.authority = authority;
        game3.score = 0;
        game3.winner = Pubkey::default();

        let game4 = &mut ctx.accounts.game4;
        game4.authority = authority;
        game4.score = 0;
        game4.winner = Pubkey::default();

        let game5 = &mut ctx.accounts.game5;
        game5.authority = authority;
        game5.score = 0;
        game5.winner = Pubkey::default();

        let game6 = &mut ctx.accounts.game6;
        game6.authority = authority;
        game6.score = 0;
        game6.winner = Pubkey::default();

        let game7 = &mut ctx.accounts.game7;
        game7.authority = authority;
        game7.score = 0;
        game7.winner = Pubkey::default();

        let game8 = &mut ctx.accounts.game8;
        game8.authority = authority;
        game8.score = 0;
        game8.winner = Pubkey::default();

        let game9 = &mut ctx.accounts.game9;
        game9.authority = authority;
        game9.score = 0;
        game9.winner = Pubkey::default();

        Ok(())
    }

    pub fn update_game1(ctx: Context<UpdateGame1>, score: u64, winner: Pubkey) -> ProgramResult {
        let game = &mut ctx.accounts.game1;

        if game.score < score {
            game.score = score;
            game.winner = winner;
        }

        Ok(())
    }
    pub fn update_game2(ctx: Context<UpdateGame2>, score: u64, winner: Pubkey) -> ProgramResult {
        let game2 = &mut ctx.accounts.game2;

        if game2.score < score {
            game2.score = score;
            game2.winner = winner;
        }

        Ok(())
    }
    pub fn update_game3(ctx: Context<UpdateGame3>, score: u64, winner: Pubkey) -> ProgramResult {
        let game3 = &mut ctx.accounts.game3;

        if game3.score < score {
            game3.score = score;
            game3.winner = winner;
        }

        Ok(())
    }
    pub fn update_game4(ctx: Context<UpdateGame4>, score: u64, winner: Pubkey) -> ProgramResult {
        let game4 = &mut ctx.accounts.game4;

        if game4.score < score {
            game4.score = score;
            game4.winner = winner;
        }

        Ok(())
    }
    pub fn update_game5(ctx: Context<UpdateGame5>, score: u64, winner: Pubkey) -> ProgramResult {
        let game5 = &mut ctx.accounts.game5;

        if game5.score < score {
            game5.score = score;
            game5.winner = winner;
        }

        Ok(())
    }
    pub fn update_game6(ctx: Context<UpdateGame6>, score: u64, winner: Pubkey) -> ProgramResult {
        let game6 = &mut ctx.accounts.game6;

        if game6.score < score {
            game6.score = score;
            game6.winner = winner;
        }

        Ok(())
    }
    pub fn update_game7(ctx: Context<UpdateGame7>, score: u64, winner: Pubkey) -> ProgramResult {
        let game7 = &mut ctx.accounts.game7;

        if game7.score < score {
            game7.score = score;
            game7.winner = winner;
        }

        Ok(())
    }
    pub fn update_game8(ctx: Context<UpdateGame8>, score: u64, winner: Pubkey) -> ProgramResult {
        let game8 = &mut ctx.accounts.game8;

        if game8.score < score {
            game8.score = score;
            game8.winner = winner;
        }

        Ok(())
    }
    pub fn update_game9(ctx: Context<UpdateGame9>, score: u64, winner: Pubkey) -> ProgramResult {
        let game9 = &mut ctx.accounts.game9;

        if game9.score < score {
            game9.score = score;
            game9.winner = winner;
        }

        Ok(())
    }
    pub fn clear_game1(ctx: Context<UpdateGame1>) -> ProgramResult {
        let game = &mut ctx.accounts.game1;

        game.score = 0;
        game.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game2(ctx: Context<UpdateGame2>) -> ProgramResult {
        let game2 = &mut ctx.accounts.game2;

        game2.score = 0;
        game2.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game3(ctx: Context<UpdateGame3>) -> ProgramResult {
        let game3 = &mut ctx.accounts.game3;

        game3.score = 0;
        game3.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game4(ctx: Context<UpdateGame4>) -> ProgramResult {
        let game4 = &mut ctx.accounts.game4;

        game4.score = 0;
        game4.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game5(ctx: Context<UpdateGame5>) -> ProgramResult {
        let game5 = &mut ctx.accounts.game5;

        game5.score = 0;
        game5.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game6(ctx: Context<UpdateGame6>) -> ProgramResult {
        let game6 = &mut ctx.accounts.game6;

        game6.score = 0;
        game6.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game7(ctx: Context<UpdateGame7>) -> ProgramResult {
        let game7 = &mut ctx.accounts.game7;

        game7.score = 0;
        game7.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game8(ctx: Context<UpdateGame8>) -> ProgramResult {
        let game8 = &mut ctx.accounts.game8;

        game8.score = 0;
        game8.winner = Pubkey::default();

        Ok(())
    }
    pub fn clear_game9(ctx: Context<UpdateGame9>) -> ProgramResult {
        let game9 = &mut ctx.accounts.game9;

        game9.score = 0;
        game9.winner = Pubkey::default();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user)]
    pub game1: Account<'info, Game1>,
    #[account(init, payer = user)]
    pub game2: Account<'info, Game2>,
    #[account(init, payer = user)]
    pub game3: Account<'info, Game3>,
    #[account(init, payer = user)]
    pub game4: Account<'info, Game4>,
    #[account(init, payer = user)]
    pub game5: Account<'info, Game5>,
    #[account(init, payer = user)]
    pub game6: Account<'info, Game6>,
    #[account(init, payer = user)]
    pub game7: Account<'info, Game7>,
    #[account(init, payer = user)]
    pub game8: Account<'info, Game8>,
    #[account(init, payer = user)]
    pub game9: Account<'info, Game9>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGame1<'info> {
    #[account(mut, has_one = authority)]
    pub game1: Account<'info, Game1>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame2<'info> {
    #[account(mut, has_one = authority)]
    pub game2: Account<'info, Game2>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame3<'info> {
    #[account(mut, has_one = authority)]
    pub game3: Account<'info, Game3>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame4<'info> {
    #[account(mut, has_one = authority)]
    pub game4: Account<'info, Game4>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame5<'info> {
    #[account(mut, has_one = authority)]
    pub game5: Account<'info, Game5>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame6<'info> {
    #[account(mut, has_one = authority)]
    pub game6: Account<'info, Game6>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame7<'info> {
    #[account(mut, has_one = authority)]
    pub game7: Account<'info, Game7>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame8<'info> {
    #[account(mut, has_one = authority)]
    pub game8: Account<'info, Game8>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateGame9<'info> {
    #[account(mut, has_one = authority)]
    pub game9: Account<'info, Game9>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame1<'info> {
    #[account(mut, has_one = authority)]
    pub game1: Account<'info, Game1>,
    #[account(
        constraint = authority.key() == game1.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame2<'info> {
    #[account(mut, has_one = authority)]
    pub game2: Account<'info, Game2>,
    #[account(
        constraint = authority.key() == game2.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame3<'info> {
    #[account(mut, has_one = authority)]
    pub game3: Account<'info, Game3>,
    #[account(
        constraint = authority.key() == game3.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame4<'info> {
    #[account(mut, has_one = authority)]
    pub game4: Account<'info, Game4>,
    #[account(
        constraint = authority.key() == game4.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame5<'info> {
    #[account(mut, has_one = authority)]
    pub game5: Account<'info, Game5>,
    #[account(
        constraint = authority.key() == game5.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame6<'info> {
    #[account(mut, has_one = authority)]
    pub game6: Account<'info, Game6>,
    #[account(
        constraint = authority.key() == game6.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame7<'info> {
    #[account(mut, has_one = authority)]
    pub game7: Account<'info, Game7>,
    #[account(
        constraint = authority.key() == game7.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame8<'info> {
    #[account(mut, has_one = authority)]
    pub game8: Account<'info, Game8>,
    #[account(
        constraint = authority.key() == game8.authority,
    )]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClearGame9<'info> {
    #[account(mut, has_one = authority)]
    pub game9: Account<'info, Game9>,
    #[account(
        constraint = authority.key() == game9.authority,
    )]
    pub authority: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Game1 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game2 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game3 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game4 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game5 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game6 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game7 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game8 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game9 {
    pub authority: Pubkey,
    pub score: u64,
    pub winner: Pubkey,
}
