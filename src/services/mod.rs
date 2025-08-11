use gloo::storage::{LocalStorage, Storage};
use gloo::net::http::Request;
use crate::models::{Statement, AnalysisQuery, AnalysisResponse};
use wasm_bindgen::prelude::*;
use web_sys::{File, FileReader};
use wasm_bindgen_futures::JsFuture;

pub struct StorageService;

impl StorageService {
    pub fn save_statement(statement: &Statement) -> Result<(), Box<dyn std::error::Error>> {
        LocalStorage::set("current_statement", statement)?;
        Ok(())
    }
    
    pub fn load_statement() -> Option<Statement> {
        LocalStorage::get("current_statement").ok()
    }
    
    pub fn clear_statement() {
        LocalStorage::delete("current_statement");
    }
}

pub struct OcrService;

impl OcrService {
    pub async fn parse_pdf(file: File) -> Result<Statement, Box<dyn std::error::Error>> {
        // For now, return mock data - in production this would call an OCR API
        let mock_statement = Statement {
            rows: vec![
                crate::models::StatementRow {
                    date: "2024-01-15".to_string(),
                    description: "Grocery Store".to_string(),
                    amount: -89.50,
                    category: Some("Food".to_string()),
                },
                crate::models::StatementRow {
                    date: "2024-01-16".to_string(),
                    description: "Salary Deposit".to_string(),
                    amount: 3000.00,
                    category: Some("Income".to_string()),
                },
                crate::models::StatementRow {
                    date: "2024-01-17".to_string(),
                    description: "Gas Station".to_string(),
                    amount: -45.75,
                    category: Some("Transport".to_string()),
                },
            ],
            total: 2864.75,
            file_name: file.name(),
        };
        
        Ok(mock_statement)
    }
}

pub struct AnalysisService;

impl AnalysisService {
    fn build_context_prompt(statement: &Statement, query: &str) -> String {
        let statement_summary = Self::generate_statement_summary(statement);
        
        format!(
            r#"You are a financial analysis expert assistant. You have access to a user's bank statement data and should provide insightful, accurate, and actionable financial advice.

CONTEXT:
- Statement file: {}
- Total balance: ${:.2}
- Number of transactions: {}
- Date range: {} to {}

STATEMENT DATA:
{}

FINANCIAL DEFINITIONS:
- Income: Positive amounts (deposits, salary, refunds, etc.)
- Expenses: Negative amounts (purchases, withdrawals, fees, etc.)
- Categories: Transactions are grouped into spending categories (Food, Transport, Entertainment, etc.)
- Cash Flow: The movement of money in and out of the account
- Net Position: Total income minus total expenses over the period

ANALYSIS GUIDELINES:
1. Provide specific insights based on the actual data
2. Compare spending patterns across categories
3. Identify unusual transactions or spending spikes
4. Suggest actionable improvements
5. Use percentages and concrete numbers from the data
6. Be concise but comprehensive in your analysis

USER QUERY: {}

Please analyze the statement data and provide insights relevant to the user's question. Include specific numbers, trends, and actionable recommendations."#,
            statement.file_name,
            statement.total,
            statement.rows.len(),
            statement.rows.first().map(|r| &r.date).unwrap_or("N/A"),
            statement.rows.last().map(|r| &r.date).unwrap_or("N/A"),
            statement_summary,
            query
        )
    }
    
    fn generate_statement_summary(statement: &Statement) -> String {
        let mut summary = String::new();
        
        // Calculate income vs expenses
        let (total_income, total_expenses): (f64, f64) = statement.rows.iter()
            .partition::<Vec<_>, _>(|row| row.amount > 0)
            .into_iter()
            .map(|rows| rows.iter().map(|r| r.amount.abs()).sum::<f64>())
            .collect::<Vec<f64>>()
            .into_iter()
            .collect::<(f64, f64)>();
        
        summary.push_str(&format!("Total Income: ${:.2}\n", total_income));
        summary.push_str(&format!("Total Expenses: ${:.2}\n", total_expenses));
        summary.push_str(&format!("Net Cash Flow: ${:.2}\n\n", total_income - total_expenses));
        
        // Group by category
        let mut category_totals: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        for row in &statement.rows {
            let category = row.category.as_ref().unwrap_or(&"Uncategorized".to_string()).clone();
            *category_totals.entry(category).or_insert(0.0) += row.amount.abs();
        }
        
        summary.push_str("SPENDING BY CATEGORY:\n");
        for (category, total) in category_totals.iter() {
            summary.push_str(&format!("- {}: ${:.2}\n", category, total));
        }
        
        summary.push_str("\nRECENT TRANSACTIONS:\n");
        for row in statement.rows.iter().take(10) {
            summary.push_str(&format!(
                "{} | {} | ${:.2} | {}\n", 
                row.date, 
                row.description, 
                row.amount,
                row.category.as_ref().unwrap_or(&"Uncategorized".to_string())
            ));
        }
        
        summary
    }
    
    pub async fn query_data(query: AnalysisQuery, statement: &Statement) -> Result<AnalysisResponse, Box<dyn std::error::Error>> {
        let context_prompt = Self::build_context_prompt(statement, &query.query);
        
        // In production, send context_prompt to your AI agent API
        // For now, provide intelligent mock responses based on the query type
        let response = Self::generate_mock_response(&query.query, statement);
        
        Ok(response)
    }
    
    fn generate_mock_response(query: &str, statement: &Statement) -> AnalysisResponse {
        let query_lower = query.to_lowercase();
        
        let (response, insights) = if query_lower.contains("spending") || query_lower.contains("expense") {
            let total_expenses: f64 = statement.rows.iter()
                .filter(|r| r.amount < 0.0)
                .map(|r| r.amount.abs())
                .sum();
            
            (
                format!("Your total expenses are ${:.2}. Food and transport make up the largest categories.", total_expenses),
                vec![
                    "Food expenses: $89.50 (highest category)".to_string(),
                    "Transport costs: $45.75 (within normal range)".to_string(),
                    "Consider meal planning to reduce food costs".to_string(),
                ]
            )
        } else if query_lower.contains("income") {
            let total_income: f64 = statement.rows.iter()
                .filter(|r| r.amount > 0.0)
                .map(|r| r.amount)
                .sum();
            
            (
                format!("Your total income is ${:.2}, primarily from salary deposits.", total_income),
                vec![
                    "Consistent income stream detected".to_string(),
                    "Good cash flow management".to_string(),
                    "Consider diversifying income sources".to_string(),
                ]
            )
        } else if query_lower.contains("budget") {
            (
                "Based on your spending patterns, here's a budget analysis:".to_string(),
                vec![
                    "50% for needs (food, transport): Currently at 48%".to_string(),
                    "30% for wants: Consider tracking entertainment expenses".to_string(),
                    "20% for savings: Good opportunity to increase savings rate".to_string(),
                ]
            )
        } else {
            (
                format!("Financial analysis for: {}", query),
                vec![
                    format!("Total transactions analyzed: {}", statement.rows.len()),
                    format!("Current balance: ${:.2}", statement.total),
                    "Use specific keywords like 'spending', 'income', or 'budget' for detailed analysis".to_string(),
                ]
            )
        };
        
        AnalysisResponse { response, insights }
    }
}