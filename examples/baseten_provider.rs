use camb_api::prelude::*;
use camb_api::provider::{BasetenProvider, TtsProvider};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("BASETEN_API_KEY")
        .expect("BASETEN_API_KEY environment variable not set");
    let url = std::env::var("BASETEN_URL").ok();

    // Initialize Custom Provider
    let provider = BasetenProvider::new(api_key, url);

    println!("Sending TTS request to Baseten...");

    // Use the provider trait
    let mut stream = provider.tts(&CreateStreamTtsRequestPayload {
        text: "Hello from Rust Custom Provider via Baseten!".to_string(),
        voice_id: 0,
        language: TtsLanguage::EnUs,
        speech_model: None,
        user_instructions: None,
        enhance_named_entities_pronunciation: None,
        output_configuration: None,
        voice_settings: None,
        inference_options: None,
    }, None).await?;

    let path = "baseten_output.mp3";
    let mut file = std::fs::File::create(path)?;
    
    while let Some(chunk) = stream.try_next().await? {
        file.write_all(&chunk)?;
    }

    println!("Success! Audio saved to {}", path);

    Ok(())
}
