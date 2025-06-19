use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleProfile {
    pub email: String,
    pub email_verified: bool,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub name: String,
    pub picture: Option<String>,
    pub sub: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleEmail {
    pub id: String,
    pub threadId: String,
    pub labelIds: Vec<String>,
    pub snippet: String,
    pub historyId: String,
    pub internalDate: Option<String>,
    pub payload: Option<GoogleEmailPayload>,
    pub sizeEstimate: Option<i64>,
    pub raw: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleEmailPayload {
    pub partId: String,
    pub mimeType: String,
    pub filename: String,
    pub headers: Option<Vec<GoogleEmailHeader>>,
    pub body: Option<GoogleEmailBody>,
    pub parts: Option<Vec<GoogleEmailPayload>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleEmailHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleEmailBody {
    pub size: i64,
    pub data: Option<String>,
    pub attachmentId: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleEmailListMail {
    pub id: String,
    pub threadId: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GoogleEmailList {
    pub messages: Vec<GoogleEmailListMail>,
    pub nextPageToken: Option<String>,
    pub resultSizeEstimate: Option<i64>,
}
