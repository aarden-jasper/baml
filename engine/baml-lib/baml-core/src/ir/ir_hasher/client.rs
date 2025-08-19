use internal_llm_client::{ClientSpec, UnresolvedClientProperty};
use ir_hasher::ClientDefinition;

use crate::ir::repr::Client;

pub struct ClientAdapter<'a> {
    pub client: &'a Client,
    pub client_spec: ClientSpec,
}

impl<'a> ClientAdapter<'a> {
    pub fn new(client: &'a Client) -> Self {
        // Create a ClientSpec based on the provider
        // Use "default" as model for now - the key is that provider changes affect the hash
        let client_spec = ClientSpec::Shorthand(client.provider.clone(), "default".to_string());
        Self {
            client,
            client_spec,
        }
    }
}

impl<'a> ClientDefinition for ClientAdapter<'a> {
    fn name(&self) -> &str {
        &self.client.name
    }

    fn spec(&self) -> &ClientSpec {
        &self.client_spec
    }

    fn retry_policy(&self) -> Option<&str> {
        self.client.retry_policy_id.as_deref()
    }

    fn options(&self) -> &UnresolvedClientProperty<()> {
        &self.client.options
    }

    fn dependencies(&self) -> Vec<String> {
        let mut deps = Vec::new();
        if let Some(retry_policy) = &self.client.retry_policy_id {
            deps.push(retry_policy.clone());
        }
        deps
    }
}
