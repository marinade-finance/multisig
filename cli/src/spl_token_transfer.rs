use std::sync::Arc;

use crate::input_pubkey::InputPubkey;
use anchor_client::solana_sdk::{pubkey::Pubkey, signature::Keypair, system_program};
use anchor_client::Program;
use anchor_spl::token::{Mint, TokenAccount};
use clap::Clap;
use spl_associated_token_account;

#[derive(Clap, Debug)]
pub struct SplTokenTransferOptions {
    #[clap(long)]
    multisig_address: InputPubkey,

    /// from account, must be a token account
    #[clap(short = 'f', long)]
    from: InputPubkey,

    /// to account, can be token or native account (ATA will be used)
    #[clap(short = 't', long)]
    to: InputPubkey,

    /// source account authority
    #[clap(long)]
    auth: InputPubkey,

    /// token amount as a decimal number (f64)
    #[clap(short = 'a', long = "amount")]
    amount_units: f64,

    /// do not create a transaction, only output base64 transaction
    #[clap(short = 'o')]
    only_output: bool,

    /// try to read or create ATA directly (when main acc is a  PDA)
    #[clap(long)]
    create_ata: bool,
}

/// propose a SPL token transfer
pub fn propose_spl_token_transfer(program: Program, args: SplTokenTransferOptions) {
    // check From account
    let from_account: TokenAccount = program
        .account(args.from.as_pubkey())
        .expect("Can not read --from account");

    println!("Mint: {}", from_account.mint);
    // here "owner" refers to token-account auth
    if from_account.owner != args.auth.as_pubkey() {
        panic!(
            "From Token-account.owner is {} but auth selected is {}",
            from_account.owner, args.auth
        );
    }

    // check --to account
    println!("--to account: {}", args.to);

    let destination: Pubkey = if args.create_ata {
        create_ata(&program, &args.to.as_pubkey(), &from_account.mint)
    } else {
        let to_account = program
            .rpc()
            .get_account(&args.to.as_pubkey())
            .expect("can not read --to account");

        // if --to account is from TokenProgram
        if to_account.owner == anchor_spl::token::ID {
            // we assume --to is a TokenAccount
            args.to.as_pubkey()
        // if _TO_ account is native, find and/or create the ATA
        } else if to_account.owner == system_program::ID {
            create_ata(&program, &args.to.as_pubkey(), &from_account.mint)
        } else {
            panic!(
                "Wrong --to account {} program-owner {}",
                args.to, to_account.owner
            )
        }
    };

    // get --from account mint
    // and convert from units to atoms (akin to SOL to lamports)
    let mint: Mint = program.account(from_account.mint).unwrap();
    let power: u64 = u64::pow(10, mint.decimals as u32);
    let atoms_amount: u64 = (args.amount_units * power as f64) as u64;

    // create the SPL-Token transfer instruction
    let instruction = spl_token::instruction::transfer(
        &anchor_spl::token::ID,
        &args.from.as_pubkey(),
        &destination,
        &args.auth.as_pubkey(),
        &[],
        atoms_amount,
    )
    .unwrap();
    println!(
        "instruction-data: {}",
        base64::encode(instruction.data.clone())
    );
    if !args.only_output {
        crate::propose_instruction(
            program,
            args.multisig_address.as_pubkey(),
            Arc::new(Keypair::new()),
            instruction,
        );
    }
}

pub fn create_ata(program: &Program, main_acc: &Pubkey, mint: &Pubkey) -> Pubkey {
    // compute ATA
    let destination = spl_associated_token_account::get_associated_token_address(&main_acc, &mint);
    println!("Destination ATA is {}", destination);
    // try read from chain
    let try_read_to_account: Result<TokenAccount, _> = program.account(destination);
    // if ata do not exists
    if try_read_to_account.is_err() {
        // Create the ATA
        println!(
            "Creating Associated Token address of {}: {}",
            &main_acc, destination
        );
        program
            .request()
            .instruction(
                spl_associated_token_account::create_associated_token_account(
                    &program.payer(),
                    &main_acc,
                    &mint,
                ),
            )
            .send()
            .expect("can not create ATA account");
    } else {
        let to_account = try_read_to_account.unwrap();
        //check same mint
        if *mint != to_account.mint {
            panic!(
                "from and to mint do not match, from mint:{} to mint:{}",
                mint, to_account.mint
            );
        }
    }
    return destination;
}
