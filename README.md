# Camb.ai Rust SDK

<div id="top" align="center">

   ![Banner](assets/banner5_720.jpg)
   <h3>
   <a href="https://camb.ai/"> Camb AI Website </a></h3>

</div>

The official Rust SDK for interacting with Camb AI's powerful voice and audio generation APIs. Create expressive speech, unique voices, and rich soundscapes with just a few lines of Rust.

## ✨ Features

- **Dubbing**: Dub your videos into multiple languages with voice cloning!
- **Expressive Text-to-Speech**: Convert text into natural-sounding speech using a wide range of pre-existing voices.
- **Generative Voices**: Create entirely new, unique voices from text prompts and descriptions.
- **Soundscapes from Text**: Generate ambient audio and sound effects from textual descriptions.
- Access to voice cloning, translation, and more (refer to full API documentation).

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
camb_api = { git = "https://github.com/Camb-ai/cambai-rust-sdk" }
tokio = { version = "1.0", features = ["full"] }
```

## 🔑 Authentication & Accessing Clients

To use the Camb AI SDK, you'll need an API key.

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("YOUR_CAMB_API_KEY".to_string()),
        ..Default::default()
    };
    let client = APIClient::new(config).expect("Failed to build client");
}
```

### Client with Specific MARS Pro Provider (e.g. Baseten)

You can use the **TtsProvider** pattern to switch between the default Camb.ai provider and custom providers like Baseten.

```rust
use camb_api::provider::{BasetenProvider, TtsProvider};

// Initialize custom provider
let tts_provider = BasetenProvider::new(
    "YOUR_BASETEN_API_KEY".to_string(),
    None, // Uses default production URL
);

// Unified interface call using the custom provider
// (See examples/baseten_provider.rs for full implementation)
```

## 🚀 Getting Started: Examples

NOTE: For more examples and full runnable files refer to the `examples/` directory.

#### TTS request options

`client.text_to_speech.tts(...)` accepts the core request fields plus optional controls for model behavior and output format:

| Field | Description |
| :--- | :--- |
| `text` | Text to synthesize. For MARS Instruct, you can include inline emotion or pacing tags. |
| `language` | Locale such as `CreateStreamTtsRequestPayloadLanguage::EnUs`. |
| `voice_id` | Voice profile ID from the voice list APIs. |
| `speech_model` | Model to use, such as `Mars8`, `Mars8Instruct`, or `Mars8Flash`. |
| `user_instructions` | Adds style, tone, pronunciation, or delivery guidance for the request. Available only with MARS Instruct. |
| `output_configuration` | Output settings such as audio format, duration, and enhancement. |
| `voice_settings` | Voice behavior controls such as reference enhancement or accent preservation. |
| `inference_options` | Advanced generation controls for supported models. |
| `enhance_named_entities_pronunciation` | Improves pronunciation for names and other named entities when supported. |

```rust
let mut stream = client.text_to_speech.tts(&CreateStreamTtsRequestPayload {
    text: "[warm, friendly] Great to meet you!".to_string(),
    voice_id: 20303,
    language: CreateStreamTtsRequestPayloadLanguage::EnUs,
    speech_model: Some(CreateStreamTtsRequestPayloadSpeechModel::Mars8Instruct),
    user_instructions: Some(Some("Speak warmly and with enthusiasm.".to_string())),
    output_configuration: Some(StreamTtsOutputConfiguration {
        format: Some(OutputFormat::Wav),
        duration: None,
        apply_enhancement: None,
    }),
    voice_settings: None,
    inference_options: None,
    enhance_named_entities_pronunciation: None,
}, None).await?;
```

### 1. Text-to-Speech (TTS)

Convert text into spoken audio using one of Camb AI's high-quality voices.

```rust
use camb_api::prelude::*;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = APIClient::new(ClientConfig {
        api_key: Some("YOUR_API_KEY".to_string()),
        ..Default::default()
    })?;

    let mut stream = client.text_to_speech.tts(&CreateStreamTtsRequestPayload {
        text: "Hello from Camb AI!".to_string(),
        voice_id: 20303,
        language: CreateStreamTtsRequestPayloadLanguage::EnUs,
        speech_model: Some(CreateStreamTtsRequestPayloadSpeechModel::Mars8),
        user_instructions: None,
        enhance_named_entities_pronunciation: None,
        output_configuration: None,
        voice_settings: None,
        inference_options: None,
    }, None).await?;

    let mut file = std::fs::File::create("output.mp3")?;
    while let Some(chunk) = stream.try_next().await? {
        file.write_all(&chunk)?;
    }

    Ok(())
}
```

### 2. Text-to-Voice (Generative Voice)

Create completely new and unique voices from a textual description.

```rust
let result = client.text_to_voice.create_text_to_voice(&CreateTextToVoiceRequestPayload {
    text: "A smooth, rich baritone voice.".to_string(),
    voice_description: Some("Ideal for storytelling.".to_string()),
    ..Default::default()
}, None).await?;
```

## License

This project is licensed under the MIT License.
