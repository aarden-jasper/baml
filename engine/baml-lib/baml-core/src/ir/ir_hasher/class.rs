use std::collections::HashSet;

use baml_types::{baml_value::TypeLookups, ir_type::TypeRPC, StringOr};
use ir_hasher::{
    interfaces::{DefinitionSource, LLMRenderable, TypedObject},
    ClassDefinition, ClassField,
};

use crate::ir::repr::{Class, Field, Node};

pub struct FieldAdapter {
    pub field: Field,
    pub converted_type: TypeRPC,
    pub alias: Option<StringOr>,
    pub description: Option<StringOr>,
    pub skip: bool,
}

impl FieldAdapter {
    pub fn new(field_node: &Node<Field>, lookup: &impl TypeLookups) -> Self {
        let converted_type = field_node.elem.r#type.elem.to_rpc_type(lookup);
        Self {
            field: field_node.elem.clone(),
            converted_type,
            alias: field_node.attributes.alias().cloned(),
            description: field_node.attributes.description().cloned(),
            skip: field_node.attributes.skip(),
        }
    }
}

impl TypedObject for FieldAdapter {
    fn name(&self) -> &str {
        &self.field.name
    }

    fn r#type(&self) -> &TypeRPC {
        &self.converted_type
    }

    fn definition_source(&self) -> DefinitionSource {
        DefinitionSource::CompileTime
    }
}

impl LLMRenderable for FieldAdapter {
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

impl ClassField for FieldAdapter {}

pub struct ClassAdapter {
    pub class: Class,
    pub converted_fields: Vec<FieldAdapter>,
    pub class_type: TypeRPC,
    pub alias: Option<StringOr>,
    pub description: Option<StringOr>,
    pub skip: bool,
}

impl ClassAdapter {
    pub fn new(class_node: &Node<Class>, lookup: &impl TypeLookups) -> Self {
        let converted_fields = class_node
            .elem
            .static_fields
            .iter()
            .map(|field_node| FieldAdapter::new(field_node, lookup))
            .collect();

        // Check if class has @dynamic attribute
        let dynamic = class_node.attributes.dynamic();

        // Create the class type
        let class_type = TypeRPC::Class {
            name: class_node.elem.name.clone(),
            mode: baml_types::StreamingMode::NonStreaming,
            dynamic,
            meta: Default::default(),
        };

        Self {
            class: class_node.elem.clone(),
            converted_fields,
            class_type,
            alias: class_node.attributes.alias().cloned(),
            description: class_node.attributes.description().cloned(),
            skip: class_node.attributes.skip(),
        }
    }
}

impl TypedObject for ClassAdapter {
    fn name(&self) -> &str {
        &self.class.name
    }

    fn r#type(&self) -> &TypeRPC {
        &self.class_type
    }

    fn definition_source(&self) -> DefinitionSource {
        DefinitionSource::CompileTime
    }
}

impl LLMRenderable for ClassAdapter {
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

impl ClassDefinition for ClassAdapter {
    fn fields_sorted_by_order<'a>(&'a self) -> Vec<&'a dyn ClassField> {
        self.converted_fields
            .iter()
            .map(|field| field as &dyn ClassField)
            .collect()
    }
}
