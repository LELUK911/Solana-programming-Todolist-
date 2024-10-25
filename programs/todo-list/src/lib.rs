use anchor_lang::prelude::*;

declare_id!("ExBDAj83MoWBR7vDV1gASKsas6QLwQhx7GiMZWEfmwZD");

#[program]
mod todo_list {
    use super::*;

    pub fn inizialize_user_list(_ctx: Context<InitializeUser>) -> Result<()> {
        let user_list = &mut _ctx.accounts.user_account;
        let signer = &_ctx.accounts.user.key();

        user_list.user = *signer;

        // Inizializza gli array vuoti
        user_list.titles = Default::default();
        user_list.descriptions = Default::default();
        user_list.dates = Default::default();
        user_list.status = Default::default();
        user_list.len = 0;

        Ok(())
    }

    pub fn add_element_user_list(
        _ctx: Context<UpdateList>,
        title: String,
        descript: String,
        data: String,
    ) -> Result<()> {
        let signer = &_ctx.accounts.user.key();
        let user_list = &mut _ctx.accounts.user_account;

        if *signer != user_list.user {
            return Err(ProgramError::IllegalOwner.into());
        }

        // Aggiungi un nuovo elemento se c'è spazio
        let index = user_list.len as usize;
        if index >= 10 {
            return Err(ProgramError::AccountDataTooSmall.into());
        }

        user_list.titles[index] = title;
        user_list.descriptions[index] = descript;
        user_list.dates[index] = data;
        user_list.status[index] = true;

        user_list.len += 1;
        msg!("Todo aggiunto con successo");
        Ok(())
    }

    pub fn turn_off_element(_ctx: Context<UpdateList>, title: String) -> Result<()> {
        let signer = &_ctx.accounts.user.key();
        let user_list = &mut _ctx.accounts.user_account;

        if *signer != user_list.user {
            return Err(ProgramError::IllegalOwner.into());
        }

        for i in 0..user_list.len {
            if user_list.titles[i as usize] == title {
                user_list.status[i as usize] = false; // Task completato
                msg!("Task completato");
                return Ok(());
            }
        }

        Err(ProgramError::InvalidArgument.into()) // Task non trovato
    }

    pub fn delete_element(_ctx: Context<UpdateList>, title: String) -> Result<()> {
        let signer = &_ctx.accounts.user.key();
        let user_list = &mut _ctx.accounts.user_account;

        if *signer != user_list.user {
            return Err(ProgramError::IllegalOwner.into());
        }

        for i in 0..user_list.len {
            if user_list.titles[i as usize] == title {
                user_list.titles[i as usize] = String::new();
                user_list.descriptions[i as usize] = String::new();
                user_list.dates[i as usize] = String::new();
                user_list.status[i as usize] = false;
                msg!("Task eliminato");
                return Ok(());
            }
        }

        Err(ProgramError::InvalidArgument.into()) // Task non trovato
    }
}

#[account]
pub struct UserList {
    pub user: Pubkey,
    pub titles: [String; 10],
    pub descriptions: [String; 10],
    pub dates: [String; 10],
    pub status: [bool; 10],
    pub len: u8,
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = user, space = 8 + 32 + (10 * 32 * 3) + (10 * 1), seeds = [b"user_account", user.key().as_ref()], bump)]
    pub user_account: Account<'info, UserList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateList<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserList>,
    #[account(mut)]
    pub user: Signer<'info>,
}
