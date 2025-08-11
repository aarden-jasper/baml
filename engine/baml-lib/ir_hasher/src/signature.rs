use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use anyhow::Result;

use crate::{
    class::{ClassDefinition, TopClassDefinition},
    client::{ClientDefinition, TopClientDefinition},
    function::TopFunctionDefinition,
    interfaces::ShallowSignature,
    r#enum::{EnumDefinition, TopEnumDefinition},
    retry_policy::{RetryPolicyDefinition, TopRetryPolicyDefinition},
    type_alias::{TopTypeAliasDefinition, TypeAliasDefinition},
};

pub struct Top<'a> {
    pub r#type: TopType<'a>,
    display_name: String,
    interface_hash: u64,
    implementation_hash: u64,
    dependencies: Vec<String>,
}

pub enum TopType<'a> {
    Function(TopFunctionDefinition<'a>),
    Class(TopClassDefinition<'a>),
    Enum(TopEnumDefinition<'a>),
    TypeAlias(TopTypeAliasDefinition<'a>),
    Client(TopClientDefinition<'a>),
    RetryPolicy(TopRetryPolicyDefinition<'a>),
}

fn recursively_collect_dependencies<'a>(
    name: &str,
    all_shallow_hashes: &'a HashMap<&str, ShallowHash>,
    find_dependencies: fn(
        name: &str,
        all_shallow_hashes: &'a HashMap<&str, ShallowHash>,
    ) -> Option<Vec<&'a String>>,
) -> Result<Vec<&'a String>> {
    // Recursively collect all dependencies
    let mut seen = HashSet::new();
    let mut queue = vec![name];
    while let Some(name) = queue.pop() {
        let dep_hash = find_dependencies(name, all_shallow_hashes)
            .ok_or(anyhow::anyhow!("Dependency: {} not found", name))?;
        for dep in dep_hash {
            // For recursive dependencies, we want to insert self back into the queue
            // This is why seen starts empty, so we actually insert the dependency
            // back into the queue, when/if we see it again
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

impl<'a> Top<'a> {
    pub fn new_function(definition: TopFunctionDefinition<'a>) -> Self {
        Self {
            display_name: definition.0.name().to_string(),
            interface_hash: definition.shallow_interface_hash(),
            implementation_hash: definition.0.shallow_implementation_hash().hash(),
            dependencies: vec![],
            r#type: TopType::Function(definition),
        }
    }

    pub fn new_class(definition: &'a dyn ClassDefinition) -> Self {
        Self::Class(TopClassDefinition::new(definition))
    }

    pub fn new_enum(definition: &'a dyn EnumDefinition) -> Self {
        Self::Enum(TopEnumDefinition::new(definition))
    }

    pub fn new_type_alias(definition: &'a dyn TypeAliasDefinition) -> Self {
        Self::TypeAlias(TopTypeAliasDefinition::new(definition))
    }

    pub fn new_client(definition: &'a dyn ClientDefinition) -> Self {
        Self::Client(TopClientDefinition::new(definition))
    }

    pub fn new_retry_policy(definition: &'a dyn RetryPolicyDefinition) -> Self {
        Self::RetryPolicy(TopRetryPolicyDefinition::new(definition))
    }
}
