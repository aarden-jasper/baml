use std::collections::HashSet;

use baml_types::{baml_value::TypeLookups, ir_type::TypeRPC, JinjaExpression};
use internal_llm_client::ClientSpec;
use ir_hasher::FunctionDefinition;

use crate::ir::repr::Function;

pub struct FunctionAdapter<'a> {
    pub function: &'a Function,
    pub converted_inputs: Vec<(String, TypeRPC)>,
    pub converted_output: TypeRPC,
    pub converted_prompt: JinjaExpression,
}

impl<'a> FunctionAdapter<'a> {
    pub fn new(function: &'a Function, lookup: &impl TypeLookups) -> Self {
        let converted_inputs = function
            .inputs
            .iter()
            .map(|(name, type_ir)| (name.clone(), type_ir.to_rpc_type(lookup)))
            .collect();
        let converted_output = function.output.to_rpc_type(lookup);

        let default_config = function
            .configs
            .iter()
            .find(|config| config.name == function.default_config)
            .expect("Default config should exist");
        let converted_prompt = JinjaExpression(default_config.prompt_template.clone());

        Self {
            function,
            converted_inputs,
            converted_output,
            converted_prompt,
        }
    }
}

impl<'a> FunctionDefinition for FunctionAdapter<'a> {
    fn name(&self) -> &str {
        self.function.name.as_str()
    }

    fn parameters(&self) -> Vec<(&str, &TypeRPC)> {
        self.converted_inputs
            .iter()
            .map(|(name, type_rpc)| (name.as_str(), type_rpc))
            .collect()
    }

    fn return_type(&self) -> &TypeRPC {
        &self.converted_output
    }

    fn prompt_config(&self) -> (&ClientSpec, &JinjaExpression) {
        let default_config = self
            .function
            .configs
            .iter()
            .find(|config| config.name == self.function.default_config)
            .expect("Default config should exist");

        (&default_config.client, &self.converted_prompt)
    }
}
