use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementRow {
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    pub rows: Vec<StatementRow>,
    pub total: f64,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisQuery {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResponse {
    pub response: String,
    pub insights: Vec<String>,
}