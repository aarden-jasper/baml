use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_generate_signatures_for_simple_ir() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

class Person {
  name string
  age int
}

enum Status {
  ACTIVE
  INACTIVE
}

function ProcessPerson(person: Person) -> Status {
  client TestClient
  prompt #"
    Process person
  "#
}
"##;
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;

    assert_eq!(
        signatures.len(),
        4,
        "Should have 4 signatures (including client)"
    );

    let names: Vec<String> = signatures.iter().map(|s| s.display_name.clone()).collect();
    assert!(names.contains(&"Person".to_string()));
    assert!(names.contains(&"Status".to_string()));
    assert!(names.contains(&"ProcessPerson".to_string()));

    Ok(())
}

#[test]
fn test_signature_includes_all_types() -> Result<()> {
    let baml = r##"
class MyClass {
  field string
}

enum MyEnum {
  VALUE
}

client<llm> MyClient {
  provider openai
  options {
    model "gpt-4"
  }
}

function MyFunction() -> string {
  client MyClient
  prompt #"
    Hello
  "#
}

type MyAlias = string

retry_policy MyRetry {
  max_retries 3
}
"##;
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;

    assert_eq!(signatures.len(), 6, "Should have 6 signatures");

    let names: Vec<String> = signatures.iter().map(|s| s.display_name.clone()).collect();
    assert!(names.contains(&"MyClass".to_string()));
    assert!(names.contains(&"MyEnum".to_string()));
    assert!(names.contains(&"MyFunction".to_string()));
    assert!(names.contains(&"MyAlias".to_string()));
    assert!(names.contains(&"MyClient".to_string()));
    assert!(names.contains(&"MyRetry".to_string()));

    Ok(())
}

#[test]
fn test_signature_has_valid_hashes() -> Result<()> {
    let baml = r#"
class Person {
  name string
}
"#;
    let ir = parse_baml(baml)?;
    let signature = get_signature_by_name(&ir, "Person")?;

    match signature.hashes {
        baml_rpc::NodeHash::CompileTimeOnly(part) => {
            assert_ne!(
                part.interface_hash(),
                0,
                "Interface hash should not be zero"
            );
            assert_ne!(
                part.impl_hash(),
                0,
                "Implementation hash should not be zero"
            );
        }
        _ => panic!("Unexpected hash type"),
    }

    Ok(())
}

#[test]
fn test_signature_dependency_tracking() -> Result<()> {
    let baml = r##"
enum Status {
  ACTIVE
  INACTIVE
}

class Person {
  name string
  status Status
}

function ProcessPerson(person: Person) -> Status {
  client "openai/gpt-4o"
  prompt #"
    Process person
  "#
}
"##;
    let ir = parse_baml(baml)?;

    // Person should depend on Status
    let person_sig = get_signature_by_name(&ir, "Person")?;
    assert!(
        person_sig.dependencies.contains(&"Status".to_string()),
        "Person should depend on Status"
    );

    // ProcessPerson should depend on both Person and Status
    let func_sig = get_signature_by_name(&ir, "ProcessPerson")?;
    assert!(
        func_sig.dependencies.contains(&"Person".to_string()),
        "ProcessPerson should depend on Person"
    );
    assert!(
        func_sig.dependencies.contains(&"Status".to_string()),
        "ProcessPerson should depend on Status"
    );

    // Status should have no dependencies
    let status_sig = get_signature_by_name(&ir, "Status")?;
    assert!(
        status_sig.dependencies.is_empty(),
        "Status should have no dependencies"
    );

    Ok(())
}

#[test]
fn test_empty_signature_generation() -> Result<()> {
    let baml = "";
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;
    assert_eq!(
        signatures.len(),
        0,
        "Empty BAML should generate no signatures"
    );
    Ok(())
}

#[test]
fn test_signature_type_correct() -> Result<()> {
    let baml = r##"
class MyClass { field string }
enum MyEnum { VALUE }
function MyFunction() -> string { prompt #"Hello"# }
type MyAlias = string
client<llm> MyClient { 
  provider openai 
  options { model "gpt-4" } 
}
retry_policy MyRetry { max_retries 3 }
"##;
    let ir = parse_baml(baml)?;
    let signatures = get_signatures(&ir)?;

    for sig in signatures {
        match sig.display_name.as_str() {
            "MyClass" => assert!(matches!(sig.r#type, ir_hasher::SignatureType::Class)),
            "MyEnum" => assert!(matches!(sig.r#type, ir_hasher::SignatureType::Enum)),
            "MyFunction" => assert!(matches!(sig.r#type, ir_hasher::SignatureType::Function)),
            "MyAlias" => assert!(matches!(sig.r#type, ir_hasher::SignatureType::TypeAlias)),
            "MyClient" => assert!(matches!(sig.r#type, ir_hasher::SignatureType::Client)),
            "MyRetry" => assert!(matches!(sig.r#type, ir_hasher::SignatureType::RetryPolicy)),
            _ => panic!("Unexpected signature name: {}", sig.display_name),
        }
    }

    Ok(())
}
