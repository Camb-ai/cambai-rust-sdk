use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TextToAudioClient {
    pub http_client: HttpClient,
}

impl TextToAudioClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_text_to_audio(
        &self,
        request: &CreateTextToAudioRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineCallResult, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "text-to-sound",
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_text_to_audio_status(
        &self,
        task_id: &String,
        request: &GetTextToAudioStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("text-to-sound/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve the Text-to-Audio (TTA) result for a given run.
    ///
    /// This endpoint validates the provided run ID and retrieves the associated
    /// Text-to-Audio output. It supports two output formats:
    /// - RAW_BYTES: Streams the audio file in WAV format.
    /// - FILE_URL: Returns a pre-signed URL for downloading the audio file.
    ///
    /// The endpoint ensures that the run type is valid for Text-to-Audio processing
    /// and that the storage preferences are applied accordingly.
    ///
    /// Args:
    /// run_id (int): Unique identifier for the TTA run.
    /// traceparent (Optional[str]): Trace header for distributed tracing.
    /// api_key_obj (dict): API key object containing authentication and storage preferences.
    /// output_type (OutputType): Desired output format. Defaults to RAW_BYTES.
    ///
    /// Returns:
    /// StreamingResponse | GetTTAOut:
    /// - StreamingResponse: If output_type is RAW_BYTES, a streaming response with the audio in WAV format.
    /// - GetTTAOut: If output_type is FILE_URL, a pre-signed URL to access the audio file.
    ///
    /// Raises:
    /// HTTPException:
    /// - 400 BAD REQUEST: If the run type is invalid for a TTA run.
    /// - 500 INTERNAL SERVER ERROR: If the audio file cannot be fetched or streamed.
    ///
    /// # Arguments
    ///
    /// * `output_type` - Output format for the Text To Speech result
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
    pub async fn get_text_to_audio_result(
        &self,
        run_id: &Option<i64>,
        request: &GetTextToAudioResultQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("text-to-sound-result/{}", run_id),
                None,
                QueryBuilder::new()
                    .string("output_type", request.output_type.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_text_to_sound_results(
        &self,
        request: &GetTextToSoundResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, TextToAudioResult>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "text-to-sound-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
