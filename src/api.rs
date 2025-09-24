use gemini_rs;

pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        gemini_rs::chat("gemini-2.5-flash-lite")
            .send_message("Hello?")
            .await?
    );
    
    Ok(())
}