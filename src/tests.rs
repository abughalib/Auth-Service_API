// use super::*;

#[cfg(test)]
mod tests {
    use actix_web::{http, test};

    use super::super::*;
    use actix_rt;

    #[actix_rt::test]
    async fn test_inedx_ok() {
        let req = test::TestRequest::default().insert_header(("content-type", "text/plain")).to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
    #[actix_rt::test]
    async fn test_account_creation_page() {}
}
