/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.8.0-alpha.2
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServiceLogoutUrl {
    /// LogoutToken can be used to perform logout using AJAX.
    #[serde(rename = "logout_token")]
    pub logout_token: String,
    /// LogoutURL can be opened in a browser to sign the user out.  format: uri
    #[serde(rename = "logout_url")]
    pub logout_url: String,
}

impl SelfServiceLogoutUrl {
    pub fn new(logout_token: String, logout_url: String) -> SelfServiceLogoutUrl {
        SelfServiceLogoutUrl {
            logout_token,
            logout_url,
        }
    }
}


