use gemini_rust::Gemini;

pub async fn get_gemini_response() -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("GEMINI_API_KEY")?;
    let client = Gemini::new(api_key)?;

    let response = client
        .generate_content()
        .with_system_prompt("You are a helpful assistant.")
        .with_user_message("Hello, how are you?")
        .execute()
        .await?;

    Ok(response.text().to_string())
}