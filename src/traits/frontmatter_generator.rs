use std::collections::HashMap;

use chrono::{DateTime, Utc};
use notion_client::{
    endpoints::{databases::query::request::{
        QueryDatabaseRequest, 
        Sort,
        SortDirection
    }, Client}, 
    objects::page::{
        Page, 
        PageProperty
    }};

use crate::NotionToObsidianError;


pub trait FrontmatterGenerator: Send + Sync {
    fn generate(&self, page: &Page, client: &Client) -> Result<String, NotionToObsidianError>;
}


pub struct DefaultFrontmatterGenerator;

impl FrontmatterGenerator for DefaultFrontmatterGenerator {
    fn generate(&self, page: &Page, client: &Client) -> Result<String, NotionToObsidianError> {
        // if let Some(PageProperty::Title { id, title }) = page.properties.get("名前") {
        //     if !title.is_empty() {
        //         Ok(format!("---\ntitle: {}\n---\n", title))
        //     } else {
        //         Err(NotionToObsidianError::NoTitleError)
        //     }
        // } else {
        //     Err(NotionToObsidianError::NoTitleError)
        // }

        let mut frontmatter = String::from("---\n");

        let formatted_time = format_datetime(page.created_time);
        frontmatter.push_str(&format!("created: {}\n", formatted_time));

        frontmatter.push_str("---\n");

        Ok(frontmatter)
    }

}


pub struct MyFrontmatterGenerator {
    tag_mapping: HashMap<String, String>,
}

impl MyFrontmatterGenerator {
    pub async fn new(tag_database_id: &str, token: String) -> MyFrontmatterGenerator {
        let client = Client::new(token, None).unwrap();
        let tag_mapping = Self::load_tags(tag_database_id, &client).await.unwrap();
        MyFrontmatterGenerator { tag_mapping }
    }

    pub async fn load_tags(tag_database_id: &str, client: &Client) -> Result<HashMap<String, String>, NotionToObsidianError> {
        let mut tag_mapping = HashMap::new();

        let mut request = QueryDatabaseRequest::default();
        request.sorts = Some(vec![Sort::Property {
            property: "名前".to_string(),
            direction: SortDirection::Ascending,
        }]);

        let response = client
            .databases
            .query_a_database(tag_database_id, request)
            .await
            .map_err(|e| NotionToObsidianError::ConversionError(e.to_string()))?;

        for page in response.results {
            if let Some(PageProperty::Title { title, .. }) = page.properties.get("名前") {
                if !title.is_empty() {
                    if let Some(tag_name) = title[0].plain_text() {
                        tag_mapping.insert(page.id.to_string(), tag_name);
                    }
                }
            }
        }

        Ok(tag_mapping)
    }

    fn extract_types(&self, page: &Page) -> Option<Vec<String>> {
        for (_, prop) in &page.properties {
            if let PageProperty::Relation { relation, .. } = prop {
                return Some(
                    relation
                        .iter()
                        .filter_map(|r| self.tag_mapping.get(&r.id).cloned())
                        .collect(),
                );
            }
        }
        None
    }
}

impl FrontmatterGenerator for MyFrontmatterGenerator {
    fn generate(&self, page: &Page, client: &Client) -> Result<String, NotionToObsidianError> {


        let mut frontmatter = String::from("---\n");

        // タイプ（タグ）の処理
        frontmatter.push_str("types:\n");
        if let Some(types) = self.extract_types(page) {
            for type_name in types {
                frontmatter.push_str(&format!("  - \"[[{}]]\"\n", type_name));
            }
        }

        // URLの処理
        if let Some(url) = extract_url(page) {
            frontmatter.push_str(&format!("URL: {}\n", url));
        }

        // 作成日時の処理
        let formatted_time = format_datetime(page.created_time);
        frontmatter.push_str(&format!("created: {}\n", formatted_time));

        frontmatter.push_str("---\n");
        Ok(frontmatter)
    }
}

fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.with_timezone(&chrono::Local)
        .format("%Y-%m-%d %H:%M")
        .to_string()
}

fn extract_url(page: &Page) -> Option<String> {
    for (_, prop) in &page.properties {
        if let PageProperty::Url { url, .. } = prop {
            return url.clone();
        }
    }
    None
}