use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::JsonObject;

use super::security::{SecurityRequirement, SecurityScheme};

/// Agent discovery document served from `/.well-known/agent-card.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCard {
    /// Human-readable agent name.
    pub name: String,
    /// Human-readable agent description.
    pub description: String,
    /// Ordered list of supported transport bindings.
    pub supported_interfaces: Vec<AgentInterface>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional agent provider metadata.
    pub provider: Option<AgentProvider>,
    /// Agent implementation version string.
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional human-readable documentation URL.
    pub documentation_url: Option<String>,
    /// Capability flags and advertised extensions.
    pub capabilities: AgentCapabilities,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    /// Named security schemes referenced by requirements.
    pub security_schemes: BTreeMap<String, SecurityScheme>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Security requirements that apply by default.
    pub security_requirements: Vec<SecurityRequirement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Default accepted input modes.
    pub default_input_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Default produced output modes.
    pub default_output_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Skills exposed by the agent.
    pub skills: Vec<AgentSkill>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Optional signatures over the agent card.
    pub signatures: Vec<AgentCardSignature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional icon URL for UI presentation.
    pub icon_url: Option<String>,
}

/// Transport binding advertised by an agent card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentInterface {
    /// Absolute or relative interface URL.
    pub url: String,
    /// Binding name such as `JSONRPC` or `HTTP+JSON`.
    pub protocol_binding: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional tenant associated with the interface.
    pub tenant: Option<String>,
    /// Protocol version served from the interface.
    pub protocol_version: String,
}

/// Organization metadata for the agent provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentProvider {
    /// Provider homepage URL.
    pub url: String,
    /// Provider or organization name.
    pub organization: String,
}

/// Capability flags and extension declarations for an agent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether streaming operations are supported.
    pub streaming: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether push-notification configuration APIs are supported.
    pub push_notifications: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Protocol extensions advertised by the agent.
    pub extensions: Vec<AgentExtension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Whether `GetExtendedAgentCard` is supported.
    pub extended_agent_card: Option<bool>,
}

/// Extension declaration inside `AgentCapabilities`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentExtension {
    /// Stable extension URI.
    pub uri: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    /// Human-readable extension description.
    pub description: String,
    #[serde(default, skip_serializing_if = "crate::types::is_false")]
    /// Whether the extension is required to interoperate.
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional extension-specific parameters.
    pub params: Option<JsonObject>,
}

/// Skill advertised by an agent card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSkill {
    /// Stable skill identifier.
    pub id: String,
    /// Human-readable skill name.
    pub name: String,
    /// Human-readable skill description.
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Searchable skill tags.
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Example prompts or invocations for the skill.
    pub examples: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Input modes supported by the skill.
    pub input_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Output modes produced by the skill.
    pub output_modes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Security requirements specific to the skill.
    pub security_requirements: Vec<SecurityRequirement>,
}

/// Signature over an agent card payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCardSignature {
    /// Protected JOSE header segment.
    pub protected: String,
    /// Signature bytes encoded as a string.
    pub signature: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Optional unprotected JOSE header values.
    pub header: Option<JsonObject>,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{AgentCapabilities, AgentCard, AgentExtension, AgentInterface, AgentSkill};

    #[test]
    fn agent_card_round_trip_serialization() {
        let card = AgentCard {
            name: "Echo Agent".to_owned(),
            description: "Replies with the same text".to_owned(),
            supported_interfaces: vec![AgentInterface {
                url: "https://example.com/rpc".to_owned(),
                protocol_binding: "JSONRPC".to_owned(),
                tenant: None,
                protocol_version: "1.0".to_owned(),
            }],
            provider: None,
            version: "0.1.0".to_owned(),
            documentation_url: None,
            capabilities: AgentCapabilities {
                streaming: Some(true),
                push_notifications: Some(false),
                extensions: vec![AgentExtension {
                    uri: "https://example.com/ext/streaming".to_owned(),
                    description: "Streaming support".to_owned(),
                    required: false,
                    params: None,
                }],
                extended_agent_card: Some(false),
            },
            security_schemes: BTreeMap::new(),
            security_requirements: Vec::new(),
            default_input_modes: vec!["text/plain".to_owned()],
            default_output_modes: vec!["text/plain".to_owned()],
            skills: vec![AgentSkill {
                id: "echo".to_owned(),
                name: "Echo".to_owned(),
                description: "Echo back user input".to_owned(),
                tags: vec!["utility".to_owned()],
                examples: vec!["echo hello".to_owned()],
                input_modes: vec!["text/plain".to_owned()],
                output_modes: vec!["text/plain".to_owned()],
                security_requirements: Vec::new(),
            }],
            signatures: Vec::new(),
            icon_url: None,
        };

        let json = serde_json::to_string(&card).expect("card should serialize");
        let round_trip: AgentCard = serde_json::from_str(&json).expect("card should deserialize");

        assert_eq!(round_trip.name, "Echo Agent");
        assert_eq!(
            round_trip.supported_interfaces[0].protocol_binding,
            "JSONRPC"
        );
        assert_eq!(
            round_trip.capabilities.extensions[0].description,
            "Streaming support"
        );
        assert!(!round_trip.capabilities.extensions[0].required);
        assert_eq!(round_trip.skills[0].id, "echo");
    }
}
