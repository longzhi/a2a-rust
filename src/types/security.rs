use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRequirement {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub schemes: BTreeMap<String, StringList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SecurityScheme {
    #[serde(rename = "apiKeySecurityScheme")]
    ApiKeySecurityScheme(ApiKeySecurityScheme),
    #[serde(rename = "httpAuthSecurityScheme")]
    HttpAuthSecurityScheme(HttpAuthSecurityScheme),
    #[serde(rename = "oauth2SecurityScheme")]
    OAuth2SecurityScheme(OAuth2SecurityScheme),
    #[serde(rename = "openIdConnectSecurityScheme")]
    OpenIdConnectSecurityScheme(OpenIdConnectSecurityScheme),
    #[serde(rename = "mtlsSecurityScheme")]
    MutualTlsSecurityScheme(MutualTlsSecurityScheme),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeySecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub location: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpAuthSecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2SecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub flows: OAuthFlows,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth2_metadata_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenIdConnectSecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub open_id_connect_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualTlsSecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OAuthFlows {
    AuthorizationCode(AuthorizationCodeOAuthFlow),
    ClientCredentials(ClientCredentialsOAuthFlow),
    Implicit(ImplicitOAuthFlow),
    Password(PasswordOAuthFlow),
    DeviceCode(DeviceCodeOAuthFlow),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCodeOAuthFlow {
    pub authorization_url: String,
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scopes: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "crate::types::is_false")]
    pub pkce_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCredentialsOAuthFlow {
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scopes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitOAuthFlow {
    pub authorization_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scopes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOAuthFlow {
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scopes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCodeOAuthFlow {
    pub device_authorization_url: String,
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub scopes: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        ApiKeySecurityScheme, AuthorizationCodeOAuthFlow, OAuth2SecurityScheme, OAuthFlows,
        SecurityScheme,
    };

    #[test]
    fn security_scheme_serializes_as_externally_tagged_enum() {
        let scheme = SecurityScheme::ApiKeySecurityScheme(ApiKeySecurityScheme {
            description: None,
            location: "header".to_owned(),
            name: "X-API-Key".to_owned(),
        });

        let json = serde_json::to_string(&scheme).expect("scheme should serialize");
        assert_eq!(
            json,
            r#"{"apiKeySecurityScheme":{"location":"header","name":"X-API-Key"}}"#
        );
    }

    #[test]
    fn oauth_flows_serializes_with_variant_name() {
        let mut scopes = BTreeMap::new();
        scopes.insert("read".to_owned(), "Read access".to_owned());

        let scheme = OAuth2SecurityScheme {
            description: None,
            flows: OAuthFlows::AuthorizationCode(AuthorizationCodeOAuthFlow {
                authorization_url: "https://example.com/authorize".to_owned(),
                token_url: "https://example.com/token".to_owned(),
                refresh_url: None,
                scopes,
                pkce_required: true,
            }),
            oauth2_metadata_url: None,
        };

        let json = serde_json::to_string(&scheme).expect("oauth2 scheme should serialize");
        assert!(json.contains(
            r#""authorizationCode":{"authorizationUrl":"https://example.com/authorize""#
        ));
        assert!(json.contains(r#""pkceRequired":true"#));
    }
}
