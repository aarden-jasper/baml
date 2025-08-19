use std::collections::HashSet;

use baml_types::{ir_type::TypeRPC, JinjaExpression};
use internal_llm_client::ClientSpec;

use crate::interfaces::ShallowSignature;

pub trait FunctionDefinition {
    fn name(&self) -> &str;
    fn parameters(&self) -> Vec<(&str, &TypeRPC)>;
    fn return_type(&self) -> &TypeRPC;
    fn prompt_config(&self) -> (&ClientSpec, &JinjaExpression);
}

pub struct TopFunctionDefinition<'a>(&'a dyn FunctionDefinition);

impl<'a> TopFunctionDefinition<'a> {
    pub fn new(definition: &'a dyn FunctionDefinition) -> Self {
        Self(definition)
    }
}

impl ShallowSignature for TopFunctionDefinition<'_> {
    fn shallow_hash_prefix(&self) -> &'static str {
        "function"
    }

    fn shallow_interface_hash(&self) -> impl std::hash::Hash {
        FunctionInterfaceHash(self.0)
    }

    fn unsorted_interface_dependencies(&self) -> HashSet<String> {
        self.0
            .parameters()
            .iter()
            .flat_map(|(_, r#type)| r#type.immediate_dependencies())
            .chain(self.0.return_type().immediate_dependencies())
            .collect()
    }

    fn unsorted_implementation_dependencies(&self) -> HashSet<String> {
        let mut deps = self.unsorted_interface_dependencies();
        let (client, _) = self.0.prompt_config();
        deps.extend(client.dependencies());
        // TODO: Somehow deal with template strings here.
        // they should be dependencies of the function
        deps
    }

    fn shallow_implementation_hash(&self) -> impl std::hash::Hash {
        FunctionImplementationHash(self.0)
    }
}

struct FunctionInterfaceHash<'a>(&'a dyn FunctionDefinition);

impl<'a> std::hash::Hash for FunctionInterfaceHash<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name().hash(state);
        let params = self.0.parameters();

        "parameters".hash(state);
        params.len().hash(state);
        for (name, r#type) in params {
            name.hash(state);
            r#type.shallow_interface_hash().hash(state);
        }
        "return_type".hash(state);
        self.0.return_type().shallow_interface_hash().hash(state);
    }
}

struct FunctionImplementationHash<'a>(&'a dyn FunctionDefinition);

impl<'a> std::hash::Hash for FunctionImplementationHash<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        FunctionInterfaceHash(self.0).hash(state);
        let (client, prompt) = self.0.prompt_config();
        crate::ast::client::ClientSpecHash(client).hash(state);
        "prompt".hash(state);
        prompt.hash(state);

        for (name, r#type) in self.0.parameters() {
            "parameter".hash(state);
            name.hash(state);
            r#type.shallow_implementation_hash().hash(state);
        }
        "return_type".hash(state);
        self.0
            .return_type()
            .shallow_implementation_hash()
            .hash(state);
    }
}
