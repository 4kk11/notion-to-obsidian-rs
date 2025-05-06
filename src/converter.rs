use log::info;
use notion2md::builder::NotionToMarkdownBuilder;
use notion_client::{
    endpoints::Client,
    objects::{
        block::Block,
        file::File,
        page::{Page, PageProperty},
    },
};
use regex::Regex;
use std::{fs, path::PathBuf};

use crate::{
    error::{NotionToObsidianError, Result},
    traits::{page_provider::PageProvider, post_processor::PostProcessor, FrontmatterGenerator},
};

pub struct NotionToObsidian {
    client: Client,
    obsidian_dir: PathBuf,
    frontmatter_generator: Box<dyn FrontmatterGenerator>,
    post_processor: Box<dyn PostProcessor>,
    page_provider: Box<dyn PageProvider>,
}

impl NotionToObsidian {
    pub fn new(
        token: String,
        obsidian_dir: PathBuf,
        frontmatter_generator: Box<dyn FrontmatterGenerator>,
        post_processor: Box<dyn PostProcessor>,
        page_provider: Box<dyn PageProvider>,
    ) -> Result<Self> {
        let client = Client::new(token, None)
            .map_err(|e| NotionToObsidianError::ConversionError(e.to_string()))?;

        Ok(Self {
            client,
            obsidian_dir,
            frontmatter_generator,
            post_processor,
            page_provider,
        })
    }

    pub async fn convert_page(&self, page_id: &str) -> Result<String> {
        let page = self
            .client
            .pages
            .retrieve_a_page(page_id, None)
            .await
            .map_err(|e| NotionToObsidianError::PageRetrievalError(e.to_string()))?;

        let frontmatter = self.generate_frontmatter(&page, &self.client);

        let converter = NotionToMarkdownBuilder::new(self.client.clone())
            .build();
        let content = converter.convert_page(page_id).await.map_err(|e| {
            NotionToObsidianError::ConversionError(format!(
                "Notionのページ {} の変換に失敗: {}",
                page_id, e
            ))
        })?;

        Ok(format!("{}{}", frontmatter, content))
    }

    fn generate_frontmatter(&self, page: &Page, client: &Client) -> String {
        self.frontmatter_generator
            .generate(page, client)
            .unwrap_or_else(|e| {
                info!("Frontmatterの生成に失敗: {}", e);
                String::new()
            })
    }

    pub fn extract_page_title(&self, page: &Page) -> Option<String> {
        for (_, property) in &page.properties {
            if let PageProperty::Title { title, .. } = property {
                let title_text: String = title.iter().filter_map(|rt| rt.plain_text()).collect();
                if !title_text.is_empty() {
                    return Some(title_text);
                }
            }
        }
        None
    }

    pub fn sanitize_filename(&self, filename: &str) -> String {
        let invalid_chars = Regex::new(r#"[/\\:*?"<>|]"#).unwrap();
        let multiple_spaces = Regex::new(r"\s+").unwrap();

        let sanitized = invalid_chars.replace_all(filename, "");
        let sanitized = multiple_spaces.replace_all(&sanitized, " ");
        sanitized.trim().to_string()
    }

    pub async fn save_to_file(&self, title: &str, content: &str) -> Result<()> {
        let filename = self.sanitize_filename(title);
        let filepath = self.obsidian_dir.join(format!("{}.md", filename));

        fs::write(&filepath, content)
            .map_err(|e| NotionToObsidianError::FileWriteError(e.to_string()))?;

        Ok(())
    }

    pub async fn migrate_pages(&self) -> Result<(usize, usize)> {
        println!("ページの変換を開始します...");

        let pages = self.page_provider.get_pages(&self.client).await?;
        println!("{} ページを取得しました。変換を開始します...", pages.len());

        let mut success_count = 0;
        for page in &pages {
            let title = self
                .extract_page_title(page)
                .unwrap_or_else(|| "Untitled".to_string());

            println!("ページ {} の変換を開始...", title);

            match self.convert_page(&page.id).await {
                Ok(full_content) => match self.save_to_file(&title, &full_content).await {
                    Ok(_) => match self.post_processor.process(page, &self.client).await {
                        Ok(_) => {
                            success_count += 1;
                            println!("ページを正常に変換しました: {}", title);
                        }
                        Err(e) => {
                            eprintln!("移行済みフラグの更新に失敗: {}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("ファイルの保存に失敗: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("ページの変換に失敗: {}", e);
                }
            }
        }

        Ok((success_count, pages.len()))
    }
}