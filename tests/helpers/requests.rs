use cynic::{http::ReqwestExt, Operation};
use regex::Regex;
use reqwest::{cookie::CookieStore, Url};

use super::test_app::TestApp;

impl<'a> TestApp {
    /// Uses reqwest client to send GQL request. Fails on errors & returns the unwrapped data.
    pub async fn send_graphql_request<A: 'a>(&self, query: Operation<'a, A>) -> A {
        let response = self
            .client
            .post(&format!("{}/graphql", &self.address))
            .run_graphql(query)
            .await
            .expect("Failed to send request");

        if let Some(errors) = response.errors {
            assert_eq!(errors.len(), 0, "Errors returned from server: {:?}", errors)
        }

        assert!(response.data.is_some());

        response.data.unwrap()
    }

    /// Checks if the authorization cookie is present in the cookie jar.
    pub fn auth_cookie_present(&self) -> bool {
        let cookie_url = Url::parse(&self.address).expect("failed to parse url");

        let cookie_jar = self.cookie_jar.cookies(&cookie_url);

        match cookie_jar.is_some() {
            false => false,
            true => {
                let cookies = cookie_jar
                    .unwrap()
                    .to_str()
                    .expect("Unable to parse cookies")
                    .to_string();

                let re = Regex::new(r"^auth=.+").expect("Regex does not compile");

                re.is_match(cookies.as_str())
            }
        }
    }
}
