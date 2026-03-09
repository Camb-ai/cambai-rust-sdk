use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct AudioSeparationClient {
    pub http_client: HttpClient,
}

impl AudioSeparationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_audio_separation(
        &self,
        request: &CreateAudioSeparationRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineCallResult, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "audio-separation",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_audio_separation_status(
        &self,
        task_id: &String,
        request: &GetAudioSeparationStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("audio-separation/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_audio_separation_run_info(
        &self,
        run_id: &Option<i64>,
        options: Option<RequestOptions>,
    ) -> Result<GetAudioSeparationResultOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("audio-separation-result/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn get_audio_separation_runs_results(
        &self,
        request: &GetAudioSeparationRunsResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, GetAudioSeparationResultOut>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "audio-separation-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
