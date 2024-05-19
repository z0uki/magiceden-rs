mod common;
use magiceden_rs::types::PopularCollectionsRequestArgs;

#[tokio::test]
async fn marketplace_popular_collections() {
    let client = common::setup_client();

    let request = PopularCollectionsRequestArgs::default()
        // .time_range("24h")
        .build()
        .unwrap();

    let response = client
        .marketplace()
        .popular_collections(request)
        .await
        .unwrap();
    println!("{:?}", response);
}
