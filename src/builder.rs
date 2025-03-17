use std::path::PathBuf;

use crate::{traits::{post_processor::{self, PostProcessor}, DefaultFrontmatterGenerator, FrontmatterGenerator}, NotionToObsidian};




pub struct NotionToObsidianBuilder {
    token: String,
    output_path: PathBuf,
    frontmatter_generator: Box<dyn FrontmatterGenerator>,
    post_processor: Box<dyn PostProcessor>,
}

impl NotionToObsidianBuilder {
    pub fn new(token: String) -> NotionToObsidianBuilder {
        // デフォルトでは出力先はカレントディレクトリ
        NotionToObsidianBuilder {
            token, 
            output_path: PathBuf::from("./"), 
            frontmatter_generator: Box::new(DefaultFrontmatterGenerator), 
            post_processor: Box::new(post_processor::DefaultPostProcessor)
        }
    }

    pub fn with_output_path(self, path: String) -> NotionToObsidianBuilder {
        NotionToObsidianBuilder {output_path: PathBuf::from(path), ..self}
    }

    pub fn with_frontmatter_generator(self, frontmatter_generator: Box<dyn FrontmatterGenerator>) -> NotionToObsidianBuilder {
        NotionToObsidianBuilder {frontmatter_generator, ..self}
        
    }

    pub fn with_post_processor(self, post_processor: Box<dyn PostProcessor>) -> NotionToObsidianBuilder {
        NotionToObsidianBuilder {post_processor, ..self}
    }

    pub fn build(self) -> crate::error::Result<NotionToObsidian> {
        NotionToObsidian::new(self.token, self.output_path, self.frontmatter_generator, self.post_processor)
    }
}