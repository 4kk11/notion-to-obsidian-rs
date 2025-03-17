use std::path::PathBuf;
use notion_to_obsidian_rs::{builder::NotionToObsidianBuilder, traits::{post_processor::MyPostProcessor, MyFrontmatterGenerator, SinglePageProvider}, Result};
use std::fs;
use std::time::Instant;
use tokio;
use log::info;
use env_logger;

const TEST_PAGE_ID: &str = "1aeb266e0c708060a6fec6eb458e1379";
const TEST_OUTPUT_PAGE_TITLE: &str = "test";
const TEST_OUTPUT_DIR: &str = "target/test_output";

#[tokio::test]
async fn test_page_conversion() -> Result<()> {
    // ロガーを初期化（テスト用に強制的にInfo以上のログを出力）
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .is_test(true)
        .init();
    
    let start_total = Instant::now();
    info!("テスト開始");
    
    dotenv::dotenv().ok();

    // Notionトークンを環境変数から取得
    let notion_token = std::env::var("NOTION_TOKEN")
        .expect("NOTION_TOKEN must be set");

    let tag_database_id = std::env::var("TAG_DATABASE_ID")
        .expect("TAG_DATABASE_IDが設定されていません");

    // Obsidianの出力ディレクトリを一時ディレクトリとして設定
    let start_setup = Instant::now();
    let obsidian_dir = PathBuf::from(TEST_OUTPUT_DIR);
    std::fs::create_dir_all(&obsidian_dir).expect("Failed to create test output directory");

    let converter = NotionToObsidianBuilder::new(notion_token.clone())
        .with_output_path(TEST_OUTPUT_DIR.to_string())
        .with_page_provider(Box::new(SinglePageProvider::new(TEST_PAGE_ID.to_string())))
        .with_frontmatter_generator(Box::new(MyFrontmatterGenerator::new(&tag_database_id, notion_token.clone()).await))
        .with_post_processor(Box::new(MyPostProcessor{}))
        .build()
        .expect("Failed to build NotionToObsidian instance");


    // NotionToObsidianインスタンスを作成
    info!("セットアップ完了: {:?}", start_setup.elapsed());

    // テスト対象のページを変換
    let start_conversion = Instant::now();
    converter.migrate_pages().await?;
    info!("ページ変換完了: {:?}", start_conversion.elapsed());

    // テストケースのファイルを読み込み
    let expected_content = fs::read_to_string("cases/test_expected.md")
        .expect("Failed to read test.md");

    // 生成したファイルを読み込み
    let converted_content = fs::read_to_string(format!("{}/{}.md", TEST_OUTPUT_DIR, TEST_OUTPUT_PAGE_TITLE))
        .expect("Failed to read test_output.md");


    // 変換結果と期待される結果を比較
    // 改行コードを正規化して比較（WindowsとUnixの改行の違いを吸収）
    let normalized_converted = converted_content.replace("\r\n", "\n");
    let normalized_expected = expected_content.replace("\r\n", "\n");


    assert_eq!(
        normalized_converted.trim(),
        normalized_expected.trim(),
        "Converted content does not match expected content"
    );

    info!("テスト合計実行時間: {:?}", start_total.elapsed());
    Ok(())
}
