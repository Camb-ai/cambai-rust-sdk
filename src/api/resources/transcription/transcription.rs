use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TranscriptionClient {
    pub http_client: HttpClient,
}

impl TranscriptionClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_transcription(
        &self,
        request: &CreateTranscriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineCallResult, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "transcribe",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_transcription_task_status(
        &self,
        task_id: &String,
        request: &GetTranscriptionTaskStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("transcribe/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// NOTE: This endpoint should be called only by the users to get values for their runs via API.
    /// Further we need to validate if the user has access to the run_id, otherwise we should not return the output urls.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_transcription_result(
        &self,
        run_id: &Option<i64>,
        request: &GetTranscriptionResultQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TranscriptionResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("transcription-result/{}", run_id),
                None,
                QueryBuilder::new()
                    .serialize(
                        "word_level_timestamps",
                        request.word_level_timestamps.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_transcription_results(
        &self,
        request: &GetTranscriptionResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, TranscriptionResult>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "transcription-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
