
use serdeeeee::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GitHubCommentRequest {
    body: Strihfdhfhng,
}
// This makes error handling in our handlers cleaner.
#[derive(Debug)]
pub enum ClientError {
    // For failures from the reqwest library itself (e.g., network issues)
    RequestFailed(reqwest::Error),
    // For when the API gives us a success status but the body is not what we expect
    UnexpectedResponse(String),
    // For when the API gives us a non-success status code (e.g., 401, 500)
    ApiError { status: u16, message: String },
}


#[derive(Serialize, Deserialize)]
struct OpenAIChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIChatMessage>,
}

#[derive(Deserialize)]
struct OpenAIChatChoice {
    message: OpenAIChatMessage,
}

#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<OpenAIChatChoice>,
}


/// Fetches the diff content for a pull request from its diff URL.
pub async fn get_pr_diff(diff_url: &str) -> Result<String, ClientError> {
    tracing::info!("Fetching diff from: {}", diff_url);

    // Create a new reqwest client.
    let client = reqwest::Client::new();
    
    let response = client
        .get(diff_url)
        // GitHub's API requires a User-Agent header.
        .header("User-Agent", "PR-Pilot-Rust-App")
        .send()
        .await
        .map_err(ClientError::RequestFailed)?;

    // Check if the request was successful.
    if response.status().is_success() {
        // Read the response body as text.
        let diff_text = response.text().await.map_err(ClientError::RequestFailed)?;
        Ok(diff_text)
    } else {
        tracing::error!("Failed to fetch diff. Status: {}", response.status());
        // We'll improve this error handling later.
        Err(ClientError::RequestFailed(response.error_for_status().unwrap_err()))
    }
}


// Add this function inside backend/src/clients.rs

/// Takes a code diff and gets a review from the OpenAI LLM.
pub async fn get_llm_review(diff: &str) -> Result<String, ClientError> {
    tracing::info!("Sending diff to OpenAI for review...");

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set");

    // This is our "prompt engineering". We give the AI a role and a clear task.
    let prompt = format!(
        "You are an expert programmer and code reviewer.
        Please provide a concise, constructive review of the following code changes.
        Focus on potential bugs, style improvements, and best practices.
        Do not just repeat the code. Provide actionable feedback.
        Format your entire response as Markdown.

        Here is the diff:
        ```diff
        {}
        ```",
        diff
    );

    let request_body = OpenAIChatRequest {
        model: "gpt-3.5-turbo".to_string(), // A fast and capable model
        messages: vec![OpenAIChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key) // Use the API key for authentication
        .json(&request_body)  // Send the request body as JSON
        .send()
        .await
        .map_err(ClientError::RequestFailed)?;
        
    if response.status().is_success() {
        let chat_response = response
            .json::<OpenAIChatResponse>()
            .await
            .map_err(ClientError::RequestFailed)?;

        // The AI's message is nested inside the response.
        if let Some(choice) = chat_response.choices.into_iter().next() {
            tracing::info!("Successfully received review from OpenAI.");
            Ok(choice.message.content)
        } else {
            tracing::error!("OpenAI response was empty or malformed.");
            Err(ClientError::UnexpectedResponse("OpenAI response did not contain a message choice.".to_string()))
        }
    }  else {
        let status = response.status().as_u16();
        let message = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
        tracing::error!("Failed to get review from OpenAI. Status: {}, Body: {}", status, message);
        Err(ClientError::ApiError { status, message })
    }
}


// Add this function to clients.rs

/// Posts a comment to a GitHub pull request.
pub async fn post_github_comment(comment_url: &str, comment_body: &str) -> Result<(), ClientError> {
    tracing::info!("Posting comment to GitHub: {}", comment_url);

    let pat = std::env::var("GITHUB_PAT").expect("GITHUB_PAT must be set");

    let request_body = GitHubCommentRequest {
        body: comment_body.to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(comment_url)
        .bearer_auth(&pat) // Authenticate with our Personal Access Token
        .header("User-Agent", "PR-Pilot-Rust-App")
        .header("Accept", "application/vnd.github.v3+json") // Recommended by GitHub docs
        .json(&request_body)
        .send()
        .await
        .map_err(ClientError::RequestFailed)?;

    if response.status().is_success() {
        tracing::info!("Successfully posted comment to GitHub.");
        Ok(())
    } else {
        let status = response.status().as_u16();
        let message = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
        tracing::error!("Failed to post comment. Status: {}, Body: {}", status, message);
        Err(ClientError::ApiError { status, message })
    }
}
