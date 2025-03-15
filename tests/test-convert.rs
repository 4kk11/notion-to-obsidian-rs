use std::path::PathBuf;
use notion_to_obsidian_rs::{NotionToObsidian, NotionToObsidianError, Result};
use std::fs;
use std::time::Instant;
use tokio;
use log::{info, error};
use env_logger;

const TEST_PAGE_ID: &str = "1aeb266e0c708060a6fec6eb458e1379";
const TEST_OUTPUT_PAGE_TITLE: &str = "test_output";
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

    // Obsidianの出力ディレクトリを一時ディレクトリとして設定
    let start_setup = Instant::now();
    let obsidian_dir = PathBuf::from(TEST_OUTPUT_DIR);
    std::fs::create_dir_all(&obsidian_dir).expect("Failed to create test output directory");

    // NotionToObsidianインスタンスを作成
    let converter = NotionToObsidian::new(notion_token, obsidian_dir)?;
    info!("セットアップ完了: {:?}", start_setup.elapsed());

    // テスト対象のページを変換
    let start_conversion = Instant::now();
    let converted_content = match converter.convert_page(TEST_PAGE_ID).await {
        Ok(full_content) => {
            info!("ページ変換完了: {:?}", start_conversion.elapsed());
            
            let start_save = Instant::now();
            match converter.save_to_file(TEST_OUTPUT_PAGE_TITLE, &full_content).await {
                Ok(_) => {
                    info!("ファイル保存完了: {:?}", start_save.elapsed());
                    full_content
                }
                Err(e) => {
                    error!("Failed to save converted page: {}", e);
                    return Err(NotionToObsidianError::FileWriteError(e.to_string()));
                }
            }
        }
        Err(e) => {
            error!("Failed to convert page: {}", e);
            return Err(NotionToObsidianError::ConversionError(e.to_string()));
        }
    };

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
