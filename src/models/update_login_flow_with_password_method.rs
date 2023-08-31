/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.0.0
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateLoginFlowWithPasswordMethod : Update Login Flow with Password Method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateLoginFlowWithPasswordMethod {
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Identifier is the email or username of the user trying to log in.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Method should be set to \"password\" when logging in using the identifier and password strategy.
    #[serde(rename = "method")]
    pub method: String,
    /// The user's password.
    #[serde(rename = "password")]
    pub password: String,
    /// Identifier is the email or username of the user trying to log in. This field is deprecated!
    #[serde(rename = "password_identifier", skip_serializing_if = "Option::is_none")]
    pub password_identifier: Option<String>,
}


impl UpdateLoginFlowWithPasswordMethod {
    /// Update Login Flow with Password Method
    pub fn new(identifier: String, method: String, password: String) -> UpdateLoginFlowWithPasswordMethod {
        UpdateLoginFlowWithPasswordMethod {
                csrf_token: None,
                identifier,
                method,
                password,
                password_identifier: None,
        }
    }
}


