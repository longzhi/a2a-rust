use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Wrapper used by proto JSON for repeated string values in maps.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Ordered string values.
    pub list: Vec<String>,
}

/// Security requirement mapping from scheme name to scopes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityRequirement {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// Required schemes and scope lists.
    pub schemes: BTreeMap<String, StringList>,
}

/// Supported security scheme variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SecurityScheme {
    #[serde(rename = "apiKeySecurityScheme")]
    /// API key security scheme.
    ApiKeySecurityScheme(ApiKeySecurityScheme),
    #[serde(rename = "httpAuthSecurityScheme")]
    /// HTTP auth security scheme.
    HttpAuthSecurityScheme(HttpAuthSecurityScheme),
    #[serde(rename = "oauth2SecurityScheme")]
    /// OAuth 2.0 security scheme.
    OAuth2SecurityScheme(OAuth2SecurityScheme),
    #[serde(rename = "openIdConnectSecurityScheme")]
    /// OpenID Connect discovery scheme.
    OpenIdConnectSecurityScheme(OpenIdConnectSecurityScheme),
    #[serde(rename = "mtlsSecurityScheme")]
    /// Mutual TLS security scheme.
    MutualTlsSecurityScheme(MutualTlsSecurityScheme),
}

/// API key security scheme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeySecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional description for human readers.
    pub description: Option<String>,
    /// Location of the API key, such as `header` or `query`.
    pub location: String,
    /// Header or parameter name carrying the key.
    pub name: String,
}

/// HTTP auth security scheme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpAuthSecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional description for human readers.
    pub description: Option<String>,
    /// Authentication scheme, such as `basic` or `bearer`.
    pub scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional bearer token format hint.
    pub bearer_format: Option<String>,
}

/// OAuth 2.0 security scheme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2SecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional description for human readers.
    pub description: Option<String>,
    /// Supported OAuth flow.
    pub flows: OAuthFlows,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional metadata discovery URL.
    pub oauth2_metadata_url: Option<String>,
}

/// OpenID Connect security scheme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenIdConnectSecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional description for human readers.
    pub description: Option<String>,
    /// OpenID Connect discovery URL.
    pub open_id_connect_url: String,
}

/// Mutual TLS security scheme definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualTlsSecurityScheme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional description for human readers.
    pub description: Option<String>,
}

/// Supported OAuth 2.0 flow variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OAuthFlows {
    /// Authorization code flow.
    AuthorizationCode(AuthorizationCodeOAuthFlow),
    /// Client credentials flow.
    ClientCredentials(ClientCredentialsOAuthFlow),
    /// Implicit flow.
    Implicit(ImplicitOAuthFlow),
    /// Resource owner password flow.
    Password(PasswordOAuthFlow),
    /// Device code flow.
    DeviceCode(DeviceCodeOAuthFlow),
}

/// Authorization code flow settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationCodeOAuthFlow {
    /// Authorization endpoint URL.
    pub authorization_url: String,
    /// Token endpoint URL.
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional refresh endpoint URL.
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// OAuth scopes and their descriptions.
    pub scopes: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "crate::types::is_false")]
    /// Whether PKCE is required for this flow.
    pub pkce_required: bool,
}

/// Client credentials flow settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientCredentialsOAuthFlow {
    /// Token endpoint URL.
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional refresh endpoint URL.
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// OAuth scopes and their descriptions.
    pub scopes: BTreeMap<String, String>,
}

/// Implicit flow settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplicitOAuthFlow {
    /// Authorization endpoint URL.
    pub authorization_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional refresh endpoint URL.
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// OAuth scopes and their descriptions.
    pub scopes: BTreeMap<String, String>,
}

/// Password flow settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordOAuthFlow {
    /// Token endpoint URL.
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional refresh endpoint URL.
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// OAuth scopes and their descriptions.
    pub scopes: BTreeMap<String, String>,
}

/// Device code flow settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCodeOAuthFlow {
    /// Device authorization endpoint URL.
    pub device_authorization_url: String,
    /// Token endpoint URL.
    pub token_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional refresh endpoint URL.
    pub refresh_url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// OAuth scopes and their descriptions.
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
