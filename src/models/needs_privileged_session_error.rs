/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.8.2-alpha.1
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeedsPrivilegedSessionError {
    /// The status code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// Debug information  This field is often not exposed to protect against leaking sensitive information.
    #[serde(rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<String>,
    /// Further error details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The error ID  Useful when trying to identify various errors in application logic.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Error message  The error's message.
    #[serde(rename = "message")]
    pub message: String,
    /// A human-readable reason for the error
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Points to where to redirect the user to next.
    #[serde(rename = "redirect_browser_to")]
    pub redirect_browser_to: String,
    /// The request ID  The request ID is often exposed internally in order to trace errors across service architectures. This is often a UUID.
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    /// The status description
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl NeedsPrivilegedSessionError {
    pub fn new(message: String, redirect_browser_to: String) -> NeedsPrivilegedSessionError {
        NeedsPrivilegedSessionError {
            code: None,
            debug: None,
            details: None,
            id: None,
            message,
            reason: None,
            redirect_browser_to,
            request: None,
            status: None,
        }
    }
}


