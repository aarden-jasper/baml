use baml_types::UnresolvedValue;
use internal_baml_parser_database::RetryPolicyStrategy;
use ir_hasher::RetryPolicyDefinition;

use crate::ir::repr::RetryPolicy;

pub struct RetryPolicyAdapter<'a> {
    pub retry_policy: &'a RetryPolicy,
}

impl<'a> RetryPolicyAdapter<'a> {
    pub fn new(retry_policy: &'a RetryPolicy) -> Self {
        Self { retry_policy }
    }
}

impl<'a> RetryPolicyDefinition for RetryPolicyAdapter<'a> {
    fn name(&self) -> &str {
        &self.retry_policy.name.0
    }

    fn max_retries(&self) -> u32 {
        self.retry_policy.max_retries
    }

    fn strategy(&self) -> &RetryPolicyStrategy {
        &self.retry_policy.strategy
    }

    fn options(&self) -> Vec<(String, UnresolvedValue<()>)> {
        self.retry_policy.options.clone()
    }
}
