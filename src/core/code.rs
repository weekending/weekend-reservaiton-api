use axum::http::StatusCode;

pub enum ApiCode {
    Ok,
    Created,
    AlreadyReserved,
}

impl ApiCode {
    pub fn status(&self) -> StatusCode {
        match self {
            ApiCode::Ok => StatusCode::OK,
            ApiCode::Created => StatusCode::CREATED,
            ApiCode::AlreadyReserved => StatusCode::BAD_REQUEST,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            ApiCode::Ok => "성공",
            ApiCode::Created => "생성 완료",
            ApiCode::AlreadyReserved => "이미 신청되었습니다.",
        }
    }
}
