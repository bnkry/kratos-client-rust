/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.0.0
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// ContinueWithSetOrySessionToken : Indicates that a session was issued, and the application should use this token for authenticated requests



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinueWithSetOrySessionToken {
    /// Action will always be `set_ory_session_token` set_ory_session_token ContinueWithActionSetOrySessionToken show_verification_ui ContinueWithActionShowVerificationUI
    #[serde(rename = "action")]
    pub action: ActionEnum,
    /// Token is the token of the session
    #[serde(rename = "ory_session_token")]
    pub ory_session_token: String,
}


impl ContinueWithSetOrySessionToken {
    /// Indicates that a session was issued, and the application should use this token for authenticated requests
    pub fn new(action: ActionEnum, ory_session_token: String) -> ContinueWithSetOrySessionToken {
        ContinueWithSetOrySessionToken {
                action,
                ory_session_token,
        }
    }
}

/// Action will always be `set_ory_session_token` set_ory_session_token ContinueWithActionSetOrySessionToken show_verification_ui ContinueWithActionShowVerificationUI
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionEnum {
    #[serde(rename = "set_ory_session_token")]
    SetOrySessionToken,
    #[serde(rename = "show_verification_ui")]
    ShowVerificationUi,
}

