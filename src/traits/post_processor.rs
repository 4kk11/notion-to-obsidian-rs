use std::collections::BTreeMap;
use async_trait::async_trait;
use notion_client::{endpoints::{pages::update::request::UpdatePagePropertiesRequest, Client}, objects::page::{Page, PageProperty}};

use crate::NotionToObsidianError;

#[async_trait]
pub trait PostProcessor: Send + Sync {
    async fn process(&self, page: &Page, client: &Client) -> Result<(), NotionToObsidianError>;
}

pub struct DefaultPostProcessor;

#[async_trait]
impl PostProcessor for DefaultPostProcessor {
    async fn process(&self, _page: &Page, _client: &Client) -> Result<(), NotionToObsidianError> {
        Ok(())
    }
}


pub struct MyPostProcessor {
}

#[async_trait]
impl PostProcessor for MyPostProcessor {
    async fn process(&self, page: &Page, client: &Client) -> Result<(), NotionToObsidianError> {

        // 移行済みフラグを更新
        let request = UpdatePagePropertiesRequest {
            properties: {
                let mut props = BTreeMap::new();
                props.insert(
                    "移行済み".to_string(), 
                    Some(PageProperty::Checkbox { 
                        id: None, 
                        checkbox: true, 
                    }),
                );
                props
            },
            ..Default::default()
        };

        match client.pages.update_page_properties(&page.id, request).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to update page properties: {:?}", e);
            }
        }

        Ok(())
    }
}