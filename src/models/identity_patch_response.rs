/*
 * Ory Identities API
 *
 * This is the API specification for Ory Identities with features such as registration, login, recovery, account verification, profile settings, password reset, identity management, session management, email and sms delivery, and more. 
 *
 * The version of the OpenAPI document: v1.0.0
 * Contact: office@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// IdentityPatchResponse : Response for a single identity patch



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityPatchResponse {
    /// The action for this specific patch create ActionCreate  Create this identity.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionEnum>,
    /// The identity ID payload of this patch
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The ID of this patch response, if an ID was specified in the patch.
    #[serde(rename = "patch_id", skip_serializing_if = "Option::is_none")]
    pub patch_id: Option<String>,
}

impl Default for IdentityPatchResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityPatchResponse {
    /// Response for a single identity patch
    pub fn new() -> IdentityPatchResponse {
        IdentityPatchResponse {
                action: None,
                identity: None,
                patch_id: None,
        }
    }
}

/// The action for this specific patch create ActionCreate  Create this identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActionEnum {
    #[serde(rename = "create")]
    Create,
}

