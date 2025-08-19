use std::collections::HashSet;

use baml_types::{baml_value::TypeLookups, ir_type::TypeRPC, StringOr};
use ir_hasher::{
    interfaces::{DefinitionSource, LLMRenderable, TypedObject},
    EnumDefinition, EnumValue,
};

use crate::ir::repr::{Enum, EnumValue as IREnumValue, Node};

pub struct EnumValueAdapter {
    pub value: IREnumValue,
    pub enum_name: String,
    pub value_type: TypeRPC,
    pub alias: Option<StringOr>,
    pub description: Option<StringOr>,
    pub skip: bool,
}

impl EnumValueAdapter {
    pub fn new(value_node: &Node<IREnumValue>, enum_name: String) -> Self {
        let value_type = TypeRPC::Enum {
            name: enum_name.clone(),
            dynamic: false,
            meta: Default::default(),
        };

        Self {
            value: value_node.elem.clone(),
            enum_name,
            value_type,
            alias: value_node.attributes.alias().cloned(),
            description: value_node.attributes.description().cloned(),
            skip: value_node.attributes.skip(),
        }
    }
}

impl TypedObject for EnumValueAdapter {
    fn name(&self) -> &str {
        &self.value.0
    }

    fn r#type(&self) -> &TypeRPC {
        &self.value_type
    }

    fn definition_source(&self) -> DefinitionSource {
        DefinitionSource::CompileTime
    }
}

impl LLMRenderable for EnumValueAdapter {
    fn alias(&self) -> Option<&StringOr> {
        self.alias.as_ref()
    }

    fn description(&self) -> Option<&StringOr> {
        self.description.as_ref()
    }

    fn skip(&self) -> bool {
        self.skip
    }
}

impl EnumValue for EnumValueAdapter {}

pub struct EnumAdapter {
    pub enum_def: Enum,
    pub converted_values: Vec<EnumValueAdapter>,
    pub enum_type: TypeRPC,
    pub alias: Option<StringOr>,
    pub description: Option<StringOr>,
    pub skip: bool,
}

impl EnumAdapter {
    pub fn new(enum_node: &Node<Enum>, _lookup: &impl TypeLookups) -> Self {
        let converted_values = enum_node
            .elem
            .values
            .iter()
            .map(|(value_node, _docstring)| {
                EnumValueAdapter::new(value_node, enum_node.elem.name.clone())
            })
            .collect();

        // Check if enum has @dynamic attribute
        let dynamic = enum_node.attributes.dynamic();

        let enum_type = TypeRPC::Enum {
            name: enum_node.elem.name.clone(),
            dynamic,
            meta: Default::default(),
        };

        Self {
            enum_def: enum_node.elem.clone(),
            converted_values,
            enum_type,
            alias: enum_node.attributes.alias().cloned(),
            description: enum_node.attributes.description().cloned(),
            skip: enum_node.attributes.skip(),
        }
    }
}

impl TypedObject for EnumAdapter {
    fn name(&self) -> &str {
        &self.enum_def.name
    }

    fn r#type(&self) -> &TypeRPC {
        &self.enum_type
    }

    fn definition_source(&self) -> DefinitionSource {
        DefinitionSource::CompileTime
    }
}

impl LLMRenderable for EnumAdapter {
    fn alias(&self) -> Option<&StringOr> {
        self.alias.as_ref()
    }

    fn description(&self) -> Option<&StringOr> {
        self.description.as_ref()
    }

    fn skip(&self) -> bool {
        self.skip
    }
}

impl EnumDefinition for EnumAdapter {
    fn values_sorted_by_order<'a>(&'a self) -> Vec<&'a dyn EnumValue> {
        self.converted_values
            .iter()
            .map(|value| value as &dyn EnumValue)
            .collect()
    }
}
