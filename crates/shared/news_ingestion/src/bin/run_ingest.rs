use news_ingestion::IngestionConfig;
#[tokio::main]
async fn main() {
    let config = IngestionConfig {
        reddit_subreddits: vec!["CryptoCurrency".into()],
        enable_reddit: false,
        google_query: "bitcoin".into(),
        news_api_key: Some("fffc3a7071ed407992c4b1b28322920b".into()),
        news_api_query: "bitcoin".into(),
        rpc_url: "https://eth-mainnet.public.blastapi.io".into(),
        onchain_address: "0xdAC17F958D2ee523a2206206994597C13D831ec7".into(),
        storage_path: "data/news".into(),
    };
    if let Err(e) = news_ingestion::run(config).await {
        eprintln!("Error: {}", e);
    }
}
