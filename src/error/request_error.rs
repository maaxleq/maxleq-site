use axum::response::IntoResponse;
use hyper::StatusCode;

pub struct RequestError {
    pub status: StatusCode,
    pub message: Option<String>,
}

impl IntoResponse for RequestError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message.unwrap_or(String::from(""))).into_response()
    }
}

impl RequestError {
    pub fn with_msg(mut self, msg: String) -> RequestError {
        self.message = Some(msg);
        self
    }

    pub fn internal_server_error() -> RequestError {
        RequestError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: None,
        }
    }

    pub fn not_found() -> RequestError {
        RequestError {
            status: StatusCode::NOT_FOUND,
            message: None,
        }
    }

    pub fn forbidden() -> RequestError {
        RequestError {
            status: StatusCode::FORBIDDEN,
            message: None,
        }
    }

    pub fn im_a_teapot() -> RequestError {
        RequestError {
            status: StatusCode::IM_A_TEAPOT,
            message: None,
        }
    }
}
