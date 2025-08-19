use anyhow::Result;
use insta;

use crate::helpers::*;

#[test]
fn test_snapshot_simple_class() -> Result<()> {
    let baml = r#"
class Person {
  name string
  age int
  email string?
}
"#;
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;

    // Convert signatures to a format suitable for snapshot testing
    let snapshot_data: Vec<_> = signatures
        .iter()
        .map(|sig| {
            let (interface, implementation) = match sig.hashes {
                baml_rpc::NodeHash::CompileTimeOnly(part) => {
                    (part.interface_hash(), part.impl_hash())
                }
                _ => panic!("Unexpected hash variant"),
            };

            serde_json::json!({
                "name": sig.display_name,
                "type": format!("{:?}", sig.r#type),
                "interface_hash": interface,
                "implementation_hash": implementation,
                "dependencies": sig.dependencies,
            })
        })
        .collect();

    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_simple_enum() -> Result<()> {
    let baml = r#"
enum Color {
  RED
  GREEN
  BLUE
}
"#;
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;

    let snapshot_data: Vec<_> = signatures
        .iter()
        .map(|sig| {
            let (interface, implementation) = match sig.hashes {
                baml_rpc::NodeHash::CompileTimeOnly(part) => {
                    (part.interface_hash(), part.impl_hash())
                }
                _ => panic!("Unexpected hash variant"),
            };

            serde_json::json!({
                "name": sig.display_name,
                "type": format!("{:?}", sig.r#type),
                "interface_hash": interface,
                "implementation_hash": implementation,
                "dependencies": sig.dependencies,
            })
        })
        .collect();

    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_function_with_dependencies() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

class Input {
  data string
}

class Output {
  result string
}

function Process(input: Input) -> Output {
  client TestClient
  prompt #"Process the input"#
}
"##;
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;

    let snapshot_data: Vec<_> = signatures
        .iter()
        .map(|sig| {
            let (interface, implementation) = match sig.hashes {
                baml_rpc::NodeHash::CompileTimeOnly(part) => {
                    (part.interface_hash(), part.impl_hash())
                }
                _ => panic!("Unexpected hash variant"),
            };

            serde_json::json!({
                "name": sig.display_name,
                "type": format!("{:?}", sig.r#type),
                "interface_hash": interface,
                "implementation_hash": implementation,
                "dependencies": sig.dependencies,
            })
        })
        .collect();

    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_complex_types() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

enum Status {
  ACTIVE
  INACTIVE
}

class Address {
  street string
  city string
  country string
}

class Person {
  name string
  addresses Address[]
  status Status
}

type PersonList = Person[]

function ProcessPeople(people: PersonList) -> Status {
  client TestClient
  prompt #"Process people"#
}
"##;
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;

    let snapshot_data: Vec<_> = signatures
        .iter()
        .map(|sig| {
            let (interface, implementation) = match sig.hashes {
                baml_rpc::NodeHash::CompileTimeOnly(part) => {
                    (part.interface_hash(), part.impl_hash())
                }
                _ => panic!("Unexpected hash variant"),
            };

            serde_json::json!({
                "name": sig.display_name,
                "type": format!("{:?}", sig.r#type),
                "interface_hash": interface,
                "implementation_hash": implementation,
                "dependencies": sig.dependencies,
            })
        })
        .collect();

    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

// Helper function to create snapshot data from BAML content
fn create_snapshot_from_baml(baml_content: &str) -> Result<Vec<serde_json::Value>> {
    let ir = parse_baml(baml_content)?;
    let signatures = get_signatures(&ir)?;

    let snapshot_data: Vec<_> = signatures
        .iter()
        .map(|sig| {
            let (interface, implementation) = match sig.hashes {
                baml_rpc::NodeHash::CompileTimeOnly(part) => {
                    (part.interface_hash(), part.impl_hash())
                }
                _ => panic!("Unexpected hash variant"),
            };

            serde_json::json!({
                "name": sig.display_name,
                "type": format!("{:?}", sig.r#type),
                "interface_hash": interface,
                "implementation_hash": implementation,
                "dependencies": sig.dependencies,
            })
        })
        .collect();

    Ok(snapshot_data)
}

// Comprehensive tests for all generator data projects

#[test]
fn test_snapshot_array_types() -> Result<()> {
    let baml = read_generator_baml("array_types")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_asserts() -> Result<()> {
    let baml = read_generator_baml("asserts")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_classes() -> Result<()> {
    let baml = read_generator_baml("classes")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_edge_cases() -> Result<()> {
    let baml = read_generator_baml("edge_cases")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_enums() -> Result<()> {
    let baml = read_generator_baml("enums")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_literal_types() -> Result<()> {
    let baml = read_generator_baml("literal_types")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_map_types() -> Result<()> {
    let baml = read_generator_baml("map_types")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_media_types() -> Result<()> {
    let baml = read_generator_baml("media_types")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_mixed_complex_types() -> Result<()> {
    let baml = read_generator_baml("mixed_complex_types")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_nested_structures() -> Result<()> {
    let baml = read_generator_baml("nested_structures")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_optional_nullable() -> Result<()> {
    let baml = read_generator_baml("optional_nullable")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_primitive_types() -> Result<()> {
    let baml = read_generator_baml("primitive_types")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_recursive_types() -> Result<()> {
    let baml = read_generator_baml("recursive_types")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_sample() -> Result<()> {
    let baml = read_generator_baml("sample")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_semantic_streaming() -> Result<()> {
    let baml = read_generator_baml("semantic_streaming")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_union_types_extended() -> Result<()> {
    let baml = read_generator_baml("union_types_extended")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}

#[test]
fn test_snapshot_unions() -> Result<()> {
    let baml = read_generator_baml("unions")?;
    let snapshot_data = create_snapshot_from_baml(&baml)?;
    insta::assert_yaml_snapshot!(snapshot_data);
    Ok(())
}
