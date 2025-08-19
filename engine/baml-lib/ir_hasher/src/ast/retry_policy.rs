use std::collections::HashSet;

use baml_types::UnresolvedValue;
use internal_baml_parser_database::RetryPolicyStrategy;

use crate::interfaces::ShallowSignature;

/// Find some way to hash the retry policy

pub trait RetryPolicyDefinition {
    fn name(&self) -> &str;
    fn max_retries(&self) -> u32;
    fn strategy(&self) -> &RetryPolicyStrategy;
    fn options(&self) -> Vec<(String, UnresolvedValue<()>)>;
}

pub struct TopRetryPolicyDefinition<'a>(&'a dyn RetryPolicyDefinition);

impl<'a> TopRetryPolicyDefinition<'a> {
    pub fn new(definition: &'a dyn RetryPolicyDefinition) -> Self {
        Self(definition)
    }
}

impl ShallowSignature for TopRetryPolicyDefinition<'_> {
    fn shallow_hash_prefix(&self) -> &'static str {
        "retry_policy"
    }

    fn shallow_interface_hash(&self) -> impl std::hash::Hash {
        RetryPolicyInterface(self.0)
    }

    fn unsorted_interface_dependencies(&self) -> HashSet<String> {
        HashSet::new()
    }

    fn shallow_implementation_hash(&self) -> impl std::hash::Hash {
        RetryPolicyImplementation(self.0)
    }
}

struct RetryPolicyInterface<'a, T: RetryPolicyDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: RetryPolicyDefinition + ?Sized> std::hash::Hash for RetryPolicyInterface<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name().hash(state);
        // Retry policy configuration affects the interface since it changes behavior
        self.0.max_retries().hash(state);
        self.0.strategy().hash(state);
        self.0.options().hash(state);
    }
}

struct RetryPolicyImplementation<'a, T: RetryPolicyDefinition + ?Sized>(pub(crate) &'a T);

impl<'a, T: RetryPolicyDefinition + ?Sized> std::hash::Hash for RetryPolicyImplementation<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        RetryPolicyInterface(self.0).hash(state);
        self.0.max_retries().hash(state);
        self.0.strategy().hash(state);
        self.0.options().hash(state);
    }
}
