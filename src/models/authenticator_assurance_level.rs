/*
 * Ory Kratos API
 *
 * Documentation for all public and administrative Ory Kratos APIs. Public and administrative APIs are exposed on different ports. Public APIs can face the public internet without any protection while administrative APIs should never be exposed without prior authorization. To protect the administative API port you should use something like Nginx, Ory Oathkeeper, or any other technology capable of authorizing incoming requests. 
 *
 * The version of the OpenAPI document: v0.10.1
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// AuthenticatorAssuranceLevel : The authenticator assurance level can be one of \"aal1\", \"aal2\", or \"aal3\". A higher number means that it is harder for an attacker to compromise the account.  Generally, \"aal1\" implies that one authentication factor was used while AAL2 implies that two factors (e.g. password + TOTP) have been used.  To learn more about these levels please head over to: https://www.ory.sh/kratos/docs/concepts/credentials

/// The authenticator assurance level can be one of \"aal1\", \"aal2\", or \"aal3\". A higher number means that it is harder for an attacker to compromise the account.  Generally, \"aal1\" implies that one authentication factor was used while AAL2 implies that two factors (e.g. password + TOTP) have been used.  To learn more about these levels please head over to: https://www.ory.sh/kratos/docs/concepts/credentials
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthenticatorAssuranceLevel {
    #[serde(rename = "aal0")]
    Aal0,
    #[serde(rename = "aal1")]
    Aal1,
    #[serde(rename = "aal2")]
    Aal2,
    #[serde(rename = "aal3")]
    Aal3,

}

impl ToString for AuthenticatorAssuranceLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Aal0 => String::from("aal0"),
            Self::Aal1 => String::from("aal1"),
            Self::Aal2 => String::from("aal2"),
            Self::Aal3 => String::from("aal3"),
        }
    }
}




