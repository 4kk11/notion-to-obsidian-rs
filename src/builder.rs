use std::path::PathBuf;

use crate::{
    traits::{
        page_provider::{DatabasePageProvider, PageProvider},
        post_processor::{self, PostProcessor},
        DefaultFrontmatterGenerator, FrontmatterGenerator,
    },
    NotionToObsidian,
};

pub struct NotionToObsidianBuilder {
    token: String,
    output_path: PathBuf,
    frontmatter_generator: Box<dyn FrontmatterGenerator>,
    post_processor: Box<dyn PostProcessor>,
    page_provider: Box<dyn PageProvider>,
}

impl NotionToObsidianBuilder {
    pub fn new(token: String) -> NotionToObsidianBuilder {
        // デフォルトではデータベースからの取得を想定
        NotionToObsidianBuilder {
            token,
            output_path: PathBuf::from("./"),
            frontmatter_generator: Box::new(DefaultFrontmatterGenerator),
            post_processor: Box::new(post_processor::DefaultPostProcessor),
            page_provider: Box::new(DatabasePageProvider::new("".to_string(), 100)),
        }
    }

    pub fn with_output_path(self, path: String) -> NotionToObsidianBuilder {
        NotionToObsidianBuilder {
            output_path: PathBuf::from(path),
            ..self
        }
    }

    pub fn with_frontmatter_generator(
        self,
        frontmatter_generator: Box<dyn FrontmatterGenerator>,
    ) -> NotionToObsidianBuilder {
        NotionToObsidianBuilder {
            frontmatter_generator,
            ..self
        }
    }

    pub fn with_post_processor(
        self,
        post_processor: Box<dyn PostProcessor>,
    ) -> NotionToObsidianBuilder {
        NotionToObsidianBuilder {
            post_processor,
            ..self
        }
    }

    pub fn with_page_provider(
        self,
        page_provider: Box<dyn PageProvider>,
    ) -> NotionToObsidianBuilder {
        NotionToObsidianBuilder {
            page_provider,
            ..self
        }
    }

    pub fn build(self) -> crate::error::Result<NotionToObsidian> {
        NotionToObsidian::new(
            self.token,
            self.output_path,
            self.frontmatter_generator,
            self.post_processor,
            self.page_provider,
        )
    }
}
