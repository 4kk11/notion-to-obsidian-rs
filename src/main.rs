use anyhow::Context;
use dotenv::dotenv;
use notion_to_obsidian_rs::{
    builder::NotionToObsidianBuilder,
    traits::{
        post_processor::MyPostProcessor, DatabasePageProvider, MyFrontmatterGenerator,
        SinglePageProvider,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let required_vars = [
        "NOTION_TOKEN",
        "OBSIDIAN_DIR",
        "ALL_DATABASE_ID",
        "TAG_DATABASE_ID",
    ];
    for var in required_vars {
        if std::env::var(var).is_err() {
            eprintln!("Error: {} is not set in environment variables", var);
            eprintln!("Please set all required environment variables in .env file");
            std::process::exit(1);
        }
    }

    let token = std::env::var("NOTION_TOKEN").context("NOTION_TOKENが設定されていません")?;
    let obsidian_dir = std::env::var("OBSIDIAN_DIR").context("OBSIDIAN_DIRが設定されていません")?;
    let tag_database_id =
        std::env::var("TAG_DATABASE_ID").context("TAG_DATABASE_IDが設定されていません")?;
    let database_id =
        std::env::var("ALL_DATABASE_ID").context("ALL_DATABASE_IDが設定されていません")?;

    // let mut converter = NotionToObsidian::new(
    //     token,
    //     PathBuf::from(obsidian_dir),
    // )?;

    // タグデータの読み込み
    // converter.load_tags(&tag_database_id).await?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("使用方法:");
        eprintln!("  {} --page <page_id> : 特定のページを変換", args[0]);
        eprintln!("  {} --limit <number> : 指定数のページを変換", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "--page" => {
            if args.len() < 3 {
                eprintln!("ページIDを指定してください");
                std::process::exit(1);
            }
            let page_id = &args[2];

            let converter = NotionToObsidianBuilder::new(token.clone())
                .with_output_path(obsidian_dir)
                .with_page_provider(Box::new(SinglePageProvider::new(page_id.to_string())))
                .with_frontmatter_generator(Box::new(
                    MyFrontmatterGenerator::new(&tag_database_id, token).await,
                ))
                .with_post_processor(Box::new(MyPostProcessor {}))
                .build()?;

            converter.migrate_pages().await?;
            println!("変換完了: ページを変換しました");
        }
        "--limit" => {
            if args.len() < 3 {
                eprintln!("変換するページ数を指定してください");
                std::process::exit(1);
            }
            let limit = args[2].parse::<usize>().unwrap_or(5);

            let converter = NotionToObsidianBuilder::new(token.clone())
                .with_output_path(obsidian_dir)
                .with_page_provider(Box::new(DatabasePageProvider::new(database_id, limit)))
                .with_frontmatter_generator(Box::new(
                    MyFrontmatterGenerator::new(&tag_database_id, token).await,
                ))
                .with_post_processor(Box::new(MyPostProcessor {}))
                .build()?;

            let (success_count, total_count) = converter.migrate_pages().await?;
            println!(
                "変換完了: {} / {} ページを変換しました",
                success_count, total_count
            );
        }
        _ => {
            eprintln!("不正な引数です");
            std::process::exit(1);
        }
    }

    Ok(())
}
