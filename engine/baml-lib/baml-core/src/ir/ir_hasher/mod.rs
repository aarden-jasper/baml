mod class;
mod client;
mod enum_impl;
mod function;
mod retry_policy;
mod type_alias;

use baml_types::baml_value::TypeLookups;
pub use class::*;
pub use client::*;
pub use enum_impl::*;
pub use function::*;
use ir_hasher::{
    CanMakeSignature, ClassDefinition, ClientDefinition, EnumDefinition, FunctionDefinition,
    RetryPolicyDefinition, TypeAliasDefinition,
};
pub use retry_policy::*;
pub use type_alias::*;

// Coordinating struct that owns all adapters and implements CanMakeSignature
struct IRSignatureAdapters<'a> {
    pub function_adapters: Vec<FunctionAdapter<'a>>,
    pub class_adapters: Vec<ClassAdapter>,
    pub enum_adapters: Vec<EnumAdapter>,
    pub type_alias_adapters: Vec<TypeAliasAdapter>,
    pub client_adapters: Vec<ClientAdapter<'a>>,
    pub retry_policy_adapters: Vec<RetryPolicyAdapter<'a>>,
}

impl super::IntermediateRepr {
    pub fn as_signature_adapters<'a>(&'a self) -> impl CanMakeSignature + 'a {
        IRSignatureAdapters::new(self)
    }
}

impl<'a> IRSignatureAdapters<'a> {
    pub fn new(ir: &'a super::IntermediateRepr) -> Self {
        let function_adapters = ir
            .functions
            .iter()
            .map(|f| FunctionAdapter::new(&f.elem, ir))
            .collect();

        let class_adapters = ir
            .classes
            .iter()
            .map(|c| ClassAdapter::new(c, ir))
            .collect();

        let enum_adapters = ir.enums.iter().map(|e| EnumAdapter::new(e, ir)).collect();

        let type_alias_adapters = ir
            .type_aliases
            .iter()
            .map(|t| TypeAliasAdapter::new(t, ir))
            .collect();

        let client_adapters = ir
            .clients
            .iter()
            .map(|c| ClientAdapter::new(&c.elem))
            .collect();

        let retry_policy_adapters = ir
            .retry_policies
            .iter()
            .map(|r| RetryPolicyAdapter::new(&r.elem))
            .collect();

        Self {
            function_adapters,
            class_adapters,
            enum_adapters,
            type_alias_adapters,
            client_adapters,
            retry_policy_adapters,
        }
    }
}

impl<'a> CanMakeSignature for IRSignatureAdapters<'a> {
    fn functions(&self) -> Vec<&dyn FunctionDefinition> {
        self.function_adapters
            .iter()
            .map(|f| f as &dyn FunctionDefinition)
            .collect()
    }

    fn classes(&self) -> Vec<&dyn ClassDefinition> {
        self.class_adapters
            .iter()
            .map(|c| c as &dyn ClassDefinition)
            .collect()
    }

    fn enums(&self) -> Vec<&dyn EnumDefinition> {
        self.enum_adapters
            .iter()
            .map(|e| e as &dyn EnumDefinition)
            .collect()
    }

    fn type_aliases(&self) -> Vec<&dyn TypeAliasDefinition> {
        self.type_alias_adapters
            .iter()
            .map(|t| t as &dyn TypeAliasDefinition)
            .collect()
    }

    fn clients(&self) -> Vec<&dyn ClientDefinition> {
        self.client_adapters
            .iter()
            .map(|c| c as &dyn ClientDefinition)
            .collect()
    }

    fn retry_policies(&self) -> Vec<&dyn RetryPolicyDefinition> {
        self.retry_policy_adapters
            .iter()
            .map(|r| r as &dyn RetryPolicyDefinition)
            .collect()
    }
}

// TODO: Remove this implementation once publisher is updated to use IRSignatureAdapters
impl CanMakeSignature for super::IntermediateRepr {
    fn functions(&self) -> Vec<&dyn FunctionDefinition> {
        todo!("Use IRSignatureAdapters::new(self) instead")
    }

    fn classes(&self) -> Vec<&dyn ClassDefinition> {
        todo!("Use IRSignatureAdapters::new(self) instead")
    }

    fn enums(&self) -> Vec<&dyn EnumDefinition> {
        todo!("Use IRSignatureAdapters::new(self) instead")
    }

    fn type_aliases(&self) -> Vec<&dyn TypeAliasDefinition> {
        todo!("Use IRSignatureAdapters::new(self) instead")
    }

    fn clients(&self) -> Vec<&dyn ClientDefinition> {
        todo!("Use IRSignatureAdapters::new(self) instead")
    }

    fn retry_policies(&self) -> Vec<&dyn RetryPolicyDefinition> {
        todo!("Use IRSignatureAdapters::new(self) instead")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::repr::make_test_ir_and_diagnostics;

    #[test]
    fn test_ir_signature_adapters_smoke() {
        // Create a simple test IR with a function
        let Ok((ir, diagnostics)) = make_test_ir_and_diagnostics(
            r##"
            client<llm> TestClient {
                provider openai
                options {
                    model gpt-4
                    api_key env.OPENAI_API_KEY
                }
            }
            
            function TestFunction(input: string) -> string {
                client TestClient
                prompt #"Test prompt: {{input}}"#
            }
            
            class TestClass {
                field string
            }
            
            enum TestEnum {
                Value1
                Value2
            }
            
            type TestAlias = string
            "##,
        ) else {
            panic!("Failed to create test IR");
        };

        if diagnostics.has_errors() {
            panic!("Test IR has errors: {:#?}", diagnostics.errors());
        }

        // Create the adapters
        let adapters = IRSignatureAdapters::new(&ir);

        // Test function access
        let functions = adapters.functions();
        assert!(!functions.is_empty(), "Should have at least one function");
        assert_eq!(functions[0].name(), "TestFunction");

        // Test class access
        let classes = adapters.classes();
        assert!(!classes.is_empty(), "Should have at least one class");
        assert_eq!(classes[0].name(), "TestClass");

        // Test enum access
        let enums = adapters.enums();
        assert!(!enums.is_empty(), "Should have at least one enum");
        assert_eq!(enums[0].name(), "TestEnum");

        // Test type alias access
        let type_aliases = adapters.type_aliases();
        assert!(
            !type_aliases.is_empty(),
            "Should have at least one type alias"
        );
        assert_eq!(type_aliases[0].name(), "TestAlias");

        println!("✅ All trait accesses work correctly!");
    }
}
