use internal_llm_client::{ClientSpec, UnresolvedClientProperty};

use crate::interfaces::ShallowSignature;

pub trait ClientDefinition {
    fn name(&self) -> &str;
    fn spec(&self) -> &ClientSpec;
    fn retry_policy(&self) -> Option<&str>;
    fn options(&self) -> &UnresolvedClientProperty<()>;
    fn dependencies(&self) -> Vec<String>;
}

pub struct TopClientDefinition<'a>(&'a dyn ClientDefinition);

impl<'a> TopClientDefinition<'a> {
    pub fn new(definition: &'a dyn ClientDefinition) -> Self {
        Self(definition)
    }
}

impl ShallowSignature for TopClientDefinition<'_> {
    fn shallow_hash_prefix(&self) -> &'static str {
        "client"
    }

    fn shallow_interface_hash(&self) -> impl std::hash::Hash {
        ClientInterfaceHash(self.0)
    }

    fn shallow_implementation_hash(&self) -> Option<impl std::hash::Hash> {
        Some(ClientImplementationHash(self.0))
    }

    fn unsorted_interface_dependencies(&self) -> std::collections::HashSet<String> {
        let mut deps = self.0.dependencies();
        if let Some(retry_policy) = self.0.retry_policy() {
            deps.push(retry_policy.to_string());
        }
        deps.into_iter().collect()
    }
}

struct ClientInterfaceHash<'a>(&'a dyn ClientDefinition);

impl<'a> std::hash::Hash for ClientInterfaceHash<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name().hash(state);
        ClientSpecHash(self.0.spec()).hash(state);
    }
}

pub(crate) struct ClientSpecHash<'a>(pub(crate) &'a ClientSpec);

impl<'a> std::hash::Hash for ClientSpecHash<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.0 {
            ClientSpec::Named(name) => {
                "named".hash(state);
                name.hash(state);
            }
            ClientSpec::Shorthand(client_provider, model) => {
                "shorthand".hash(state);
                client_provider.hash(state);
                model.hash(state);
            }
        }
    }
}

struct ClientImplementationHash<'a>(&'a dyn ClientDefinition);

impl<'a> std::hash::Hash for ClientImplementationHash<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ClientInterfaceHash(self.0).hash(state);
        if let Some(retry_policy) = self.0.retry_policy() {
            "retry_policy".hash(state);
            retry_policy.hash(state);
        }
        self.0.options().hash(state);
    }
}
