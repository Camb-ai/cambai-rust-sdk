use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TranslationClient {
    pub http_client: HttpClient,
}

impl TranslationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn translation_stream(
        &self,
        request: &CreateTranslationStreamRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "translation/stream",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn create_translation(
        &self,
        request: &CreateTranslationRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "translate",
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_translation_task_status(
        &self,
        task_id: &String,
        request: &GetTranslationTaskStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("translate/{}", task_id),
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
    pub async fn get_translation_result(
        &self,
        run_id: &Option<i64>,
        options: Option<RequestOptions>,
    ) -> Result<TranslationResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("translation-result/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn get_translation_results(
        &self,
        request: &GetTranslationResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, TranslationResult>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "translation-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
