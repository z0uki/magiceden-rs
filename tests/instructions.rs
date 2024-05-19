use magiceden_rs::types::InstructionsBuyRequestArgs;

mod common;

#[tokio::test]
async fn instructions_buy() {
    let client = common::setup_client();

    let request = InstructionsBuyRequestArgs::default()
        .buyer("FUKTdYGCdAJZfdZR2b5FnBR2FfrnG6UHfbtdn4syqVYk")
        .auction_house_address("E8cU1WiRWjanGxmn96ewBgk9vPTcL6AEZ1t6F6fkgUWe")
        .token_mint("762otaAyYKdrsdgHEJByD5gVQNoj58ETHkojPx3s4a3M")
        .price(21.35)
        .build()
        .unwrap();

    let response = client.instructions().buy(request).await.unwrap();
    println!("{:?}", response);
}
