use anchor_lang::prelude::*;

declare_id!("8UaMZBKh46JLSdKfYUyF8rQZ8qTq273JcS4V4WpMSshC");

#[program]
// Smart contract functions


pub mod create {

    use super::*;

    pub fn create_contactlist(ctx: Context<CreateContact>) -> Result<()> {

    // crearing contact pda function 




    }

}

pub mod conversation {
    use super::*;

    pub fn create_conversation(ctx: Context<CreatConversation>) -> Result<()> {

        pub fn create
        
    
    
    
    
        }

}

pub mod counter {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
    msg!("Creating a Counter!!");

    let counter = &mut ctx.accounts.counter;
    counter.authority = ctx.accounts.authority.key();
    counter.count = 0;

    msg!("Current count is {}", counter.count);
    msg!("The Admin PubKey is: {} ", counter.authority);

    Ok(())
}


    pub fn update_counter(ctx: Context<UpdateCounter>) -> Result<()> {
    msg!("Adding 1 to the counter!!");
    let counter = &mut ctx.accounts.counter;
    counter.count += 1 ;
    msg!("Current count is {}", counter.count);
    msg!("{} remaining to reach 1000 ", 1000 - counter.count);

    Ok(())
}

}
// ------------------------- contact pda---------------------------------------
#[derive(Accounts)]
pub struct Contact<'info> {
    #[account(mut)]
    authotiry:Signer<'info>

    #[account{
        init,
        seeds = [authority().as_ref()] , 
        bump,
        payer =                           // get users paubkey (users that call smart contracat)
    }]
}


 //___________________________***********___________________________________________
// Data validators
#[derive(Accounts)]
pub struct CreateCounter<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        init,
        seeds = [authority.key().as_ref() ],
        bump,
        
        payer = authority,
        space = 10000
    )]
    counter: Account<'info, Counter>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    counter: Account<'info, Counter>,
}


// Data structures
#[account]
pub struct Counter {
    authority: Pubkey,
    count: u64,
}

//  ------------------------------ contact structs ----------------------

#[account]
pub struct contact {
    authority: signer,
    
    private String user_name;
    
    private String last_name;
    
    private String public_key;
   
    private String base_pubkey;
    
    private String avatar;

    // private boolean is_select;

    // public boolean isFilled;
}



//  ------------------------------ conversation structs ----------------------

#[account]
pub struct conversation {
    authority: signer, 

    
    private String conversation_name;
   
    private String created_time;
    
    private List<MessageModel> messages;
    
    private List<UserModel> members;
    
    private UserModel admin;

    private boolean is_private;
}



/* Choosing Between Authority Signer and Pubkey:

The choice between setting an authority signer or public key depends on your specific use case and desired level of control:

1. Authority Signer:

    Use Case: Ideal when you want to grant write access to the PDA and its associated data.
    Functionality: The wallet associated with the signer must actively sign transactions to interact with the PDA function.
    Implementation:
        In Anchor, use the set_authority instruction with the Signer authority type.
        In Rust, leverage the solana_program::program_derivation function to create the PDA and set the signer using the set_authority API.

2. Pubkey:

    Use Case: Suitable when you need to read data from the PDA or perform actions that don't modify its state.
    Functionality: The wallet associated with the pubkey doesn't need to actively sign transactions, but its pubkey must be part of the PDA derivation process.
    Implementation:
        In Anchor, create the PDA using the program_derived_address instruction with the desired pubkeys involved in the derivation.
        In Rust, use the solana_program::program_derivation function to create the PDA without setting an authority. */