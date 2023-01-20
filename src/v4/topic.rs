use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    messages::{NTMessage, UnpublishTopic},
    Type,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct PublishedTopic {
    pub(crate) name: String,
    pub(crate) pubuid: i32,
    pub(crate) r#type: Type,
    pub(crate) properties: Option<PublishProperties>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Topic {
    pub(crate) name: String,
    pub(crate) id: i32,
    pub(crate) pubuid: Option<i32>,
    pub(crate) r#type: Type,
    pub(crate) properties: Option<PublishProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct PublishProperties {
    /// If true, the last set value will be periodically saved to persistent storage on the server and be restored during server startup.
    /// Topics with this property set to true will not be deleted by the server when the last publisher stops publishing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) persistent: Option<bool>,
    /// Topics with this property set to true will not be deleted by the server when the last publisher stops publishing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) retained: Option<bool>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub(crate) rest: Option<HashMap<String, serde_json::Value>>,
}

impl PublishedTopic {
    pub(crate) fn as_unpublish(&self) -> NTMessage {
        NTMessage::Unpublish(UnpublishTopic {
            pubuid: self.pubuid,
        })
    }
}