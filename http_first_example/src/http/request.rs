use super::method::Method;

pub struct Request {
    path: String,
    query_params: Option<String>,
    method: Method,
}
