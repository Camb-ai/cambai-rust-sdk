use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TextToSpeechClient {
    pub http_client: HttpClient,
}

impl TextToSpeechClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn tts(
        &self,
        request: &CreateStreamTtsRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::POST,
                "tts-stream",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn create_tts(
        &self,
        request: &CreateTtsRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<CreateTtsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "tts",
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_tts_result(
        &self,
        task_id: &String,
        request: &GetTtsResultQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("tts/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the result of a Text To Speech (TTS) run.
    ///
    /// This endpoint validates the provided `run_id` and fetches the corresponding TTS-generated audio.
    /// The user must have valid access to the run. The function supports two output formats:
    /// - `RAW_BYTES`: Streams the audio file directly.
    /// - `FILE_URL`: Returns a pre-signed URL to download the audio file.
    ///
    /// Args:
    /// run_id (int): Unique identifier for the TTS run.
    /// api_key_obj (dict): API key object used for authentication and storage preferences.
    /// traceparent (Optional[str]): Traceparent header for distributed tracing.
    /// output_type (OutputType, optional): Determines the output format. Defaults to `RAW_BYTES`.
    ///
    /// Returns:
    /// StreamingResponse | GetTTSOut:
    /// - If `output_type = RAW_BYTES`: A streaming response containing the TTS-generated audio in FLAC format.
    /// - If `output_type = FILE_URL`: A URL pointing to the stored TTS-generated audio file.
    ///
    /// Raises:
    /// HTTPException:
    /// - 400 BAD REQUEST if the run ID is invalid or does not belong to a TTS process.
    /// - 500 INTERNAL SERVER ERROR if fetching or streaming the audio fails.
    ///
    /// Assumptions:
    /// - The user has valid access to the `run_id`.
    /// - The `run_id` corresponds to a valid TTS run.
    /// - There is only **one** dialogue associated with the given `run_id`.
    ///
    /// # Arguments
    ///
    /// * `output_type` - Output format for the Text To Speech result
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_tts_run_info(
        &self,
        run_id: &Option<i64>,
        request: &GetTtsRunInfoQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetTtsRunInfoTtsResultRunIDGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("tts-result/{}", run_id),
                None,
                QueryBuilder::new()
                    .string("output_type", request.output_type.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_tts_results(
        &self,
        request: &GetTtsResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, GetTtsResultsTtsResultsPostResponseValue>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "tts-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_tts_result_discord(
        &self,
        task_id: &String,
        request: &GetTtsResultDiscordQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("discord/tts/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
