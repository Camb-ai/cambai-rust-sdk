use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ProjectSetupClient {
    pub http_client: HttpClient,
}

impl ProjectSetupClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a new project setup with transcription capabilities.
    ///
    /// This endpoint allows users to create a new project by providing media content
    /// (either as a file upload or URL), specifying source and target languages, and
    /// other project metadata. The function validates inputs, checks file size limitations,
    /// and initiates the project setup process.
    ///
    /// Args:
    /// request_payload (CreateProjectSetupRequestPayload): Complete project configuration
    /// including media URL, source/target languages, project metadata, and
    /// processing preferences such as audio track selection and dictionary choices.
    /// api_key_obj_and_subscription: Dependency injection providing validated API key
    /// object and associated subscription details for authorization and usage
    /// limit enforcement.
    /// traceparent (str | None, optional): OpenTelemetry trace parent header for
    /// distributed tracing across microservices. Enables request correlation
    /// and performance monitoring throughout the processing pipeline.
    ///
    /// Returns:
    /// Project setup response with project details and processing status.
    ///
    /// Raises:
    /// HTTPException:
    /// - 400: If neither media_file nor media_url is provided
    /// - 400: If uploaded file has no filename
    /// - 413: If uploaded file exceeds size limit
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_project(
        &self,
        request: &CreateProjectSetupRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<CreateProjectSetupOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "project-setup",
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_project_setup_task_status(
        &self,
        task_id: &String,
        request: &CreateProjectSetupTaskStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetCreateProjectSetupResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("project-setup/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the final result of a completed project setup.
    ///
    /// This endpoint provides access to the final results of a completed project setup.
    /// It verifies that the authenticated user has access to the requested run_id and
    /// validates that the run is of the correct type (`DUB_PROJECT`) before returning results.
    ///
    /// Note:
    /// This endpoint should only be called by users to retrieve their run results via API.
    /// Access validation is performed to ensure users can only access their own runs.
    ///
    /// Args:
    /// run_id: Positive integer ID of the project setup run.
    /// api_key_obj: API key authentication data from dependency.
    /// traceparent: OpenTelemetry trace header for distributed tracing.
    ///
    /// Returns:
    /// GetCreateProjectSetupResponse: Project setup results including run details.
    ///
    /// Raises:
    /// HTTPException:
    /// - 404: If the run_id is not found
    /// - 400: If the run type is not valid for this endpoint (must be DUB_PROJECT)
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_project_setup_result(
        &self,
        run_id: &Option<i64>,
        options: Option<RequestOptions>,
    ) -> Result<Option<GetCreateProjectSetupResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("project-setup-result/{}", run_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn get_project_setup_runs_results(
        &self,
        request: &GetProjectSetupRunsResultsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<GetCreateProjectSetupResponse>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "project-setup-results",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
