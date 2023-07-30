use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DiscussionCommentWebhookEventPayload {
    pub action: DiscussionCommentWebhookEventAction,
    pub changes: Option<serde_json::Value>,
    pub comment: serde_json::Value,
    pub discussion: serde_json::Value,
    pub enterprise: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiscussionCommentWebhookEventAction {
    Created,
    Deleted,
    Edited,
}
