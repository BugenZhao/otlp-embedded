use crate::proto::collector::trace::v1::{trace_service_server::TraceService, *};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

use crate::State;

/// The implementation of [`TraceService`].
pub struct TraceServiceImpl {
    state: Arc<RwLock<State>>,
}

impl TraceServiceImpl {
    /// Create a new [`TraceServiceImpl`] with the given [`State`] reference.
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl TraceService for TraceServiceImpl {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> std::result::Result<Response<ExportTraceServiceResponse>, Status> {
        let request = request.into_inner();

        let mut state = self.state.write().await;
        for resource_spans in request.resource_spans {
            state.apply(resource_spans);
        }

        Ok(Response::new(ExportTraceServiceResponse {
            partial_success: None,
        }))
    }
}
