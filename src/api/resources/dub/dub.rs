use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct DubClient {
    pub http_client: HttpClient,
}

impl DubClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn end_to_end_dubbing(
        &self,
        request: &EndToEndDubbingRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineCallResult, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "dub",
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_end_to_end_dubbing_status(
        &self,
        task_id: &String,
        request: &GetEndToEndDubbingStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("dub/{}", task_id),
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
    pub async fn get_dubbed_run_info(
        &self,
        run_id: &Option<i64>,
        options: Option<RequestOptions>,
    ) -> Result<GetDubbedRunInfoDubResultRunIDGetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("dub-result/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn get_dubbing_runs_results(
        &self,
        request: &GetDubbingRunsResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, GetDubbingRunsResultsDubbingResultsPostResponseValue>, ApiError>
    {
        self.http_client
            .execute_request(
                Method::POST,
                "dubbing-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_dubbed_run_transcript(
        &self,
        run_id: &Option<i64>,
        language: &Languages,
        request: &GetDubbedRunTranscriptQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("transcript/{}/{}", run_id, language.0),
                None,
                QueryBuilder::new()
                    .serialize("format_type", request.format_type.clone())
                    .serialize("data_type", request.data_type.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_dubbed_output_in_alt_format(
        &self,
        run_id: &Option<i64>,
        language: &Languages,
        request: &DubbedOutputInAltFormatRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<GetDubbedOutputInAltFormatDubAltFormatRunIDLanguagePostResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("dub-alt-format/{}/{}", run_id, language.0),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_dubbed_output_in_alt_format_status(
        &self,
        task_id: &String,
        request: &GetDubbedOutputInAltFormatStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("dub-alt-format/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn poll_discord_dub_task(
        &self,
        task_id: &String,
        request: &PollDiscordDubTaskQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("discord/dub/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn poll_twitter_dub_task(
        &self,
        task_id: &String,
        request: &PollTwitterDubTaskQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("twitter/dub/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
