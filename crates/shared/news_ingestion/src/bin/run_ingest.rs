use news_ingestion::IngestionConfig;
#[tokio::main]
async fn main() {
    let config = IngestionConfig {
        reddit_subreddits: vec!["CryptoCurrency".into()],
        google_query: "bitcoin".into(),
        news_api_key: Some("fffc3a7071ed407992c4b1b28322920b".into()),
        news_api_query: "bitcoin".into(),
        rpc_url: "http://localhost:8545".into(),
        storage_path: "data/news".into(),
    };
    if let Err(e) = news_ingestion::run(config).await {
        eprintln!("Error: {}", e);
    }
}
