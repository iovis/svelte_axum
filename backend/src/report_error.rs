use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use color_eyre::Report;

pub struct ReportError(Report);

impl std::fmt::Display for ReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // {:?} shows the backtrace/spantrace
        write!(f, "{:?}", self.0)
    }
}

impl From<Report> for ReportError {
    fn from(err: Report) -> Self {
        ReportError(err)
    }
}

impl IntoResponse for ReportError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error: {}", self.0),
        )
            .into_response()
    }
}
