use crate::service::common::tags::ApiTags;
use poem_openapi::OpenApi;

mod live_get;
mod ready_get;
mod started_get;

pub struct HealthApi;

#[OpenApi(prefix_path = "/health", tag = "ApiTags::Health")]
impl HealthApi {
    #[oai(path = "/started", method = "get", operation_id = "healthStarted")]
    /// Service Started
    ///
    /// This endpoint is used to determine if the service has started properly
    /// and is able to serve requests.
    ///
    /// ## Note
    ///
    /// *This endpoint is for internal use of the service deployment infrastructure.
    /// It may not be exposed publicly.*
    ///
    async fn started_get(&self) -> started_get::AllResponses {
        started_get::endpoint().await
    }

    #[oai(path = "/ready", method = "get", operation_id = "healthReady")]
    /// Service Ready
    ///
    /// This endpoint is used to determine if the service is ready and able to serve requests.
    ///
    /// ## Note
    ///
    /// *This endpoint is for internal use of the service deployment infrastructure.
    /// It may not be exposed publicly.*
    ///
    async fn ready_get(&self) -> ready_get::AllResponses {
        ready_get::endpoint().await
    }

    #[oai(path = "/live", method = "get", operation_id = "healthLive")]
    /// Service Live
    ///
    /// This endpoint is used to determine if the service is live.
    ///
    /// ## Note
    ///
    /// *This endpoint is for internal use of the service deployment infrastructure.
    /// It may not be exposed publicly. Refer to []*
    ///
    async fn live_get(&self) -> live_get::AllResponses {
        live_get::endpoint().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::api::tests::mk_test_app;
    use poem::http::StatusCode;

    #[tokio::test]
    async fn health_test() {
        let app = mk_test_app(HealthApi);

        let resp = app.get("/health/started").send().await;
        resp.assert_status(StatusCode::NO_CONTENT);

        let resp = app.get("/health/ready").send().await;
        resp.assert_status(StatusCode::NO_CONTENT);

        let resp = app.get("/health/live").send().await;
        resp.assert_status(StatusCode::NO_CONTENT);
    }
}
