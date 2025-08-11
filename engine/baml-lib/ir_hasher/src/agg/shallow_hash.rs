use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use anyhow::Result;

use crate::{
    ast::{
        class::{ClassDefinition, TopClassDefinition},
        client::{ClientDefinition, TopClientDefinition},
        function::{FunctionDefinition, TopFunctionDefinition},
        r#enum::{EnumDefinition, TopEnumDefinition},
        retry_policy::{RetryPolicyDefinition, TopRetryPolicyDefinition},
        type_alias::{TopTypeAliasDefinition, TypeAliasDefinition},
    },
    interfaces::ShallowSignature,
};

pub(super) struct ShallowHash {
    interface_hash: u64,
    implementation_hash: u64,
    interface_dependencies: Vec<String>,
    implementation_dependencies: Vec<String>,
}

impl ShallowHash {
    fn new<T: ShallowSignature>(signature: T) -> Self {
        Self {
            interface_hash: {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                signature.shallow_interface_hash().hash(&mut hasher);
                hasher.finish()
            },
            implementation_hash: {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                signature.shallow_implementation_hash().hash(&mut hasher);
                hasher.finish()
            },
            interface_dependencies: signature.interface_dependencies(),
            implementation_dependencies: signature.implementation_dependencies(),
        }
    }
}

pub struct AllTops<'a> {
    all_shallow_hashes: HashMap<&'a str, ShallowHash>,
}

impl<'a> AllTops<'a> {
    pub fn new(
        functions: Vec<&'a dyn FunctionDefinition>,
        classes: Vec<&'a dyn ClassDefinition>,
        enums: Vec<&'a dyn EnumDefinition>,
        type_aliases: Vec<&'a dyn TypeAliasDefinition>,
        clients: Vec<&'a dyn ClientDefinition>,
        retry_policies: Vec<&'a dyn RetryPolicyDefinition>,
    ) -> Self {
        let mut all_shallow_hashes = HashMap::new();
        for function in functions {
            all_shallow_hashes.insert(
                function.name(),
                ShallowHash::new(TopFunctionDefinition::new(function)),
            );
        }
        for class in classes {
            all_shallow_hashes.insert(
                class.name(),
                ShallowHash::new(TopClassDefinition::new(class)),
            );
        }
        for enum_ in enums {
            all_shallow_hashes.insert(
                enum_.name(),
                ShallowHash::new(TopEnumDefinition::new(enum_)),
            );
        }
        for type_alias in type_aliases {
            all_shallow_hashes.insert(
                type_alias.name(),
                ShallowHash::new(TopTypeAliasDefinition::new(type_alias)),
            );
        }
        for client in clients {
            all_shallow_hashes.insert(
                client.name(),
                ShallowHash::new(TopClientDefinition::new(client)),
            );
        }
        for retry_policy in retry_policies {
            all_shallow_hashes.insert(
                retry_policy.name(),
                ShallowHash::new(TopRetryPolicyDefinition::new(retry_policy)),
            );
        }
        Self { all_shallow_hashes }
    }

    fn find_dependencies<'b>(
        &'b self,
        name: &str,
        find_dependencies: fn(&'b Self, &str) -> Option<&'b Vec<String>>,
    ) -> Result<Vec<&'b String>> {
        // Recursively collect all dependencies
        let mut seen = HashSet::new();
        let mut queue = vec![name];
        while let Some(name) = queue.pop() {
            let dep_hash = find_dependencies(self, name)
                .ok_or(anyhow::anyhow!("Dependency: {} not found", name))?;
            for dep in dep_hash {
                if seen.insert(dep) {
                    queue.push(dep);
                }
            }
        }

        // Sort dependencies so hasher is deterministic
        let mut dependencies = seen.into_iter().collect::<Vec<_>>();
        dependencies.sort();
        Ok(dependencies)
    }

    fn fn_find_dependencies_interface(&'a self, name: &str) -> Option<&'a Vec<String>> {
        self.all_shallow_hashes
            .get(name)
            .map(|hash| &hash.interface_dependencies)
    }

    fn fn_find_dependencies_implementation(&'a self, name: &str) -> Option<&'a Vec<String>> {
        self.all_shallow_hashes
            .get(name)
            .map(|hash| &hash.implementation_dependencies)
    }

    fn find_interface_dependencies(&'a self, name: &str) -> Result<Vec<&'a String>> {
        self.find_dependencies(name, Self::fn_find_dependencies_interface)
    }

    fn find_implementation_dependencies(&'a self, name: &str) -> Result<Vec<&'a String>> {
        self.find_dependencies(name, Self::fn_find_dependencies_implementation)
    }
}
