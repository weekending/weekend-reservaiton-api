use axum::http::StatusCode;

pub trait HttpCode {
    fn status(&self) -> StatusCode;
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;
}

pub enum Http2xx {
    Ok,
    Created,
}

impl HttpCode for Http2xx {
    fn status(&self) -> StatusCode {
        match self {
            Http2xx::Ok => StatusCode::OK,
            Http2xx::Created => StatusCode::CREATED,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Http2xx::Ok => "S001",
            Http2xx::Created => "S002",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Http2xx::Ok => "성공",
            Http2xx::Created => "생성 완료",
        }
    }
}

pub enum Http4xx {
    BadRequest,
    AlreadyReserved,
}

impl HttpCode for Http4xx {
    fn status(&self) -> StatusCode {
        match self {
            Http4xx::BadRequest => StatusCode::BAD_REQUEST,
            Http4xx::AlreadyReserved => StatusCode::BAD_REQUEST,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Http4xx::BadRequest => "F001",
            Http4xx::AlreadyReserved => "F002",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Http4xx::BadRequest => "파라미터 에러",
            Http4xx::AlreadyReserved => "이미 신청되었습니다.",
        }
    }
}
