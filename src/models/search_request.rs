/*
 * Zotero Endpoint
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SearchRequest : A json payload



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "joinMode", skip_serializing_if = "Option::is_none")]
    pub join_mode: Option<JoinMode>,
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::models::SearchCondition>,
}

impl SearchRequest {
    /// A json payload
    pub fn new(conditions: Vec<crate::models::SearchCondition>) -> SearchRequest {
        SearchRequest {
            join_mode: None,
            conditions,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JoinMode {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "ALL")]
    All,
}

impl Default for JoinMode {
    fn default() -> JoinMode {
        Self::Any
    }
}

