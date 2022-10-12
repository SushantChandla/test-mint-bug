
use anchor_lang::prelude::Pubkey;
use anchor_spl::token::TokenAccount;
use solana_sdk::transaction::Transaction;
use solana_sdk::{account::Account, signature::Keypair, signer::Signer};
use solana_test_framework::{ProgramTest, *};

pub async fn mint_base_tokens(
    context: &mut ProgramTestContext,
    mint: Pubkey,
    authority: &Keypair,
    token_account: Pubkey,
    amount: u64,
) {
    let mint_ix = anchor_spl::token::spl_token::instruction::mint_to(
        &anchor_spl::token::spl_token::id(),
        &mint,
        &token_account,
        &authority.pubkey(),
        &[&authority.pubkey()],
        amount,
    )
    .expect("Failed to create mint instruction");

    let mint_tx = Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&authority.pubkey()),
        &[authority],
        context.banks_client.get_latest_blockhash().await.unwrap(),
    );

    context
        .banks_client
        .process_transaction(mint_tx)
        .await
        .expect("Failed to mint base tokens");
}

#[tokio::test]
async fn mint_test() {
    let mut program_test = ProgramTest::default();
    let mint = Keypair::new();
    let authority = Keypair::new();
    let user = Keypair::new();
    program_test.add_account(
        authority.pubkey(),
        Account {
            lamports: 1000000000000,
            ..Default::default()
        },
    );
    program_test.add_account(
        user.pubkey(),
        Account {
            lamports: 1000000000000,
            ..Default::default()
        },
    );

    let mut context = program_test.start_with_context().await;
    context
        .banks_client
        .create_token_mint(&mint, &authority.pubkey(), None, 6, &authority)
        .await
        .unwrap();

    let user_associated_token_account = context
        .banks_client
        .create_associated_token_account(&user.pubkey(), &mint.pubkey(), &user)
        .await
        .unwrap();

    let mint_amount = 1000;
    mint_base_tokens(
        &mut context,
        mint.pubkey(),
        &authority,
        user_associated_token_account,
        mint_amount,
    )
    .await;

    mint_base_tokens(
        &mut context,
        mint.pubkey(),
        &authority,
        user_associated_token_account,
        mint_amount,
    )
    .await;
    mint_base_tokens(
        &mut context,
        mint.pubkey(),
        &authority,
        user_associated_token_account,
        mint_amount,
    )
    .await;

    let token_account: TokenAccount = context
        .banks_client
        .get_account_with_anchor(user_associated_token_account)
        .await
        .unwrap();

    // THis is mint_amount
    // println!("balance {}", token_account.amount);
    assert_eq!(
        token_account.amount,
        mint_amount + mint_amount + mint_amount
    )
}
