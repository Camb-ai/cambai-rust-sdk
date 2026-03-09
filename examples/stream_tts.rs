use camb_api::prelude::*;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("CAMB_API_KEY").expect("CAMB_API_KEY environment variable not set");

    let client = APIClient::new(ClientConfig {
        api_key: Some(api_key),
        ..ClientConfig::default()
    })?;

    println!("Sending Streaming TTS request...");

    let mut stream = client
        .text_to_speech
        .tts(
            &CreateStreamTtsRequestPayload {
                text: "Hello from Camb AI Rust SDK Streaming!".to_string(),
                voice_id: 20303,
                language: CreateStreamTtsRequestPayloadLanguage::EnUs,
                speech_model: Some(CreateStreamTtsRequestPayloadSpeechModel::Mars8),
                user_instructions: None,
                enhance_named_entities_pronunciation: None,
                output_configuration: Some(StreamTtsOutputConfiguration {
                    format: Some(OutputFormat::Mp3),
                    duration: None,
                    apply_enhancement: None,
                }),
                voice_settings: None,
                inference_options: None,
            },
            None,
        )
        .await?;

    let path = "streaming_output.mp3";
    let mut file = std::fs::File::create(path)?;

    while let Some(chunk) = stream.try_next().await? {
        file.write_all(&chunk)?;
    }

    println!("Success! Streaming Audio saved to {}", path);

    Ok(())
}
