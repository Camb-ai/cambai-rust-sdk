use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TranslatedStoryClient {
    pub http_client: HttpClient,
}

impl TranslatedStoryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_translation_for_existing_story(
        &self,
        run_id: &Option<i64>,
        request: &CreateTranslationForExistingStoryRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<AddTargetLanguageOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("translated-story/{}", run_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_translated_story_status(
        &self,
        task_id: &String,
        request: &GetTranslatedStoryStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("translated-story/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_translated_story_run_info(
        &self,
        run_id: &Option<i64>,
        target_language: &Languages,
        request: &GetTranslatedStoryRunInfoQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("translated-story-result/{}/{}", run_id, target_language.0),
                None,
                QueryBuilder::new()
                    .serialize("include_transcript", request.include_transcript.clone())
                    .build(),
                options,
            )
            .await
    }
}
