use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TextToVoiceClient {
    pub http_client: HttpClient,
}

impl TextToVoiceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_text_to_voice(
        &self,
        request: &CreateTextToVoiceRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineCallResult, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "text-to-voice",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_text_to_voice_status(
        &self,
        task_id: &String,
        request: &GetTextToVoiceStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("text-to-voice/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_text_to_voice_result(
        &self,
        run_id: &Option<i64>,
        options: Option<RequestOptions>,
    ) -> Result<GetTextToVoiceResultOut, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("text-to-voice-result/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }
}
