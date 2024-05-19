use magiceden_rs::types::{CurveType, MmmCreatePoolRequestArgs};

mod common;

#[tokio::test]
async fn instructions_buy() {
    let client = common::setup_client();
    let pubkey = "FUKTdYGCdAJZfdZR2b5FnBR2FfrnG6UHfbtdn4syqVYk";
    let request = MmmCreatePoolRequestArgs::default()
        .spot_price(129000000)
        .curve_type(CurveType::Exp)
        .curve_delta(0)
        .reinvest_buy(false)
        .reinvest_sell(false)
        .expiry(0)
        .lp_fee_bp(0)
        .buyside_creator_royalty_bp(0)
        .payment_mint(pubkey)
        .collection_symbol("kingpins_nft")
        .owner(pubkey)
        .sol_deposit(129000000)
        .build()
        .unwrap();

    let response = client.mmm().create_pool(request).await.unwrap();
    println!("{:?}", response);
}
