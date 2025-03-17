use crate::error::Result;
use async_trait::async_trait;
use notion_client::{
    endpoints::{
        databases::query::request::{
            CheckBoxCondition, Filter, FilterType, PropertyCondition, QueryDatabaseRequest, Sort,
            SortDirection,
        },
        Client,
    },
    objects::page::Page,
};

#[async_trait]
pub trait PageProvider: Send + Sync {
    async fn get_pages(&self, client: &Client) -> Result<Vec<Page>>;
}

pub struct DatabasePageProvider {
    database_id: String,
    limit: usize,
}

impl DatabasePageProvider {
    pub fn new(database_id: String, limit: usize) -> Self {
        Self { database_id, limit }
    }

    fn build_query(&self) -> QueryDatabaseRequest {
        QueryDatabaseRequest {
            filter: Some(Filter::Value {
                filter_type: FilterType::Property {
                    property: "移行済み".to_string(),
                    condition: PropertyCondition::Checkbox(CheckBoxCondition::Equals(false)),
                },
            }),
            sorts: Some(vec![Sort::Property {
                property: "作成日時".to_string(),
                direction: SortDirection::Descending,
            }]),
            page_size: Some(self.limit.try_into().unwrap()),
            ..Default::default()
        }
    }
}

#[async_trait]
impl PageProvider for DatabasePageProvider {
    async fn get_pages(&self, client: &Client) -> Result<Vec<Page>> {
        let request = self.build_query();
        let response = client
            .databases
            .query_a_database(&self.database_id, request)
            .await
            .map_err(|e| crate::error::NotionToObsidianError::ConversionError(e.to_string()))?;

        Ok(response.results)
    }
}

pub struct SinglePageProvider {
    page_id: String,
}

impl SinglePageProvider {
    pub fn new(page_id: String) -> Self {
        Self { page_id }
    }
}

#[async_trait]
impl PageProvider for SinglePageProvider {
    async fn get_pages(&self, client: &Client) -> Result<Vec<Page>> {
        let page = client
            .pages
            .retrieve_a_page(&self.page_id, None)
            .await
            .map_err(|e| crate::error::NotionToObsidianError::PageRetrievalError(e.to_string()))?;

        Ok(vec![page])
    }
}
