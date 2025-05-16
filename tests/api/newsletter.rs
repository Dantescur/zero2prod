use crate::helpers::{TestApp, spawn_app};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn newsletters_are_not_deivered_to_uncorfirmed_subscribers() {
    let app = spawn_app().await;
}

async fn create_unconfirmed_subscriber(app: &TestApp) {
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let _mock_guard = Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200.))
        .named("Create uncorfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;
}
