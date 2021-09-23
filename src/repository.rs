use std::collections::HashMap;

use aws_config;
use aws_sdk_dynamodb::{
    model::{AttributeValue, Select},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Brew {
    id: String,
    brew_title: String,
}

pub struct Repository {
    dynamo_client: Client,
    table_name: &'static str,
}

impl Repository {
    pub async fn new(table_name: &'static str) -> Repository {
        let shared_config = aws_config::load_from_env().await;
        let dynamo_client = Client::new(&shared_config);

        Repository {
            dynamo_client,
            table_name,
        }
    }
}

impl Repository {
    pub async fn get_brew(&self, brew_id: &str) -> Option<Brew> {
        let response = self
            .dynamo_client
            .query()
            .table_name(self.table_name)
            .key_condition_expression("#key = :value")
            .expression_attribute_names("#key", "id")
            .expression_attribute_values(":value", AttributeValue::S(brew_id.to_string()))
            .limit(1)
            .select(Select::AllAttributes)
            .send()
            .await
            .unwrap();

        let items = match response.items {
            Some(items) => items,
            None => return None,
        };

        let item = match items.get(0) {
            Some(item) => item,
            None => return None,
        };

        Some(self.format_brew(item))
    }

    pub async fn get_brews(&self) -> Vec<Brew> {
        let response = self
            .dynamo_client
            .scan()
            .table_name(self.table_name)
            .select(Select::AllAttributes)
            .send()
            .await
            .unwrap();

        let items = match response.items {
            Some(items) => items,
            None => return Vec::new(),
        };

        let mut brews = Vec::new();

        for item in items {
            brews.push(self.format_brew(&item))
        }

        brews
    }

    pub async fn post_brew<'a>(&self, brew: &'a Brew) -> Option<&'a Brew> {
        let response = self
            .dynamo_client
            .put_item()
            .table_name(self.table_name)
            .item("id", AttributeValue::S(brew.id.to_string()))
            .item("brew_title", AttributeValue::S(brew.brew_title.to_string()))
            .send()
            .await;

        match response {
            Ok(_) => Some(brew),
            Err(_) => {
                log::error!("An error occurred inserting document");

                None
            }
        }
    }

    fn format_brew(&self, map: &HashMap<String, AttributeValue>) -> Brew {
        Brew {
            id: if let AttributeValue::S(id) = map.get("id").unwrap() {
                id.to_string()
            } else {
                "unknown".to_string()
            },
            brew_title: if let AttributeValue::S(brew_title) = map.get("brew_title").unwrap() {
                brew_title.to_string()
            } else {
                "unknown".to_string()
            },
        }
    }
}
