use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct StoryClient {
    pub http_client: HttpClient,
}

impl StoryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_story(
        &self,
        request: &CreateStoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateStoryStoryPostResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "story",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn setup_story(
        &self,
        request: &SetupStoryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetupStoryStorySetupPostResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "story-setup",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_story_status(
        &self,
        task_id: &String,
        request: &GetStoryStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("story/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_story_run_info(
        &self,
        run_id: &Option<i64>,
        request: &GetStoryRunInfoQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("story-result/{}", run_id),
                None,
                QueryBuilder::new()
                    .serialize("include_transcript", request.include_transcript.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_stories_runs_results(
        &self,
        request: &GetStoriesRunsResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, HashMap<String, serde_json::Value>>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "stories-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
