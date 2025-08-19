use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_direct_dependencies() -> Result<()> {
    let baml = r#"
enum Color {
  RED
  GREEN
  BLUE
}

class Car {
  color Color
  brand string
}
"#;
    let ir = parse_baml(baml)?;

    // Car should depend on Color
    let car_sig = get_signature_by_name(&ir, "Car")?;
    assert_eq!(car_sig.dependencies.len(), 1);
    assert!(car_sig.dependencies.contains(&"Color".to_string()));

    // Color should have no dependencies
    let color_sig = get_signature_by_name(&ir, "Color")?;
    assert!(color_sig.dependencies.is_empty());

    Ok(())
}

#[test]
fn test_transitive_dependencies() -> Result<()> {
    let baml = r#"
enum Status {
  ACTIVE
  INACTIVE
}

class Address {
  street string
  city string
}

class Person {
  name string
  address Address
  status Status
}

class Company {
  employees Person[]
  headquarters Address
}
"#;
    let ir = parse_baml(baml)?;

    // Person depends on Address and Status
    let person_sig = get_signature_by_name(&ir, "Person")?;
    assert!(person_sig.dependencies.contains(&"Address".to_string()));
    assert!(person_sig.dependencies.contains(&"Status".to_string()));

    // Company depends on Person and Address
    let company_sig = get_signature_by_name(&ir, "Company")?;
    assert!(company_sig.dependencies.contains(&"Person".to_string()));
    assert!(company_sig.dependencies.contains(&"Address".to_string()));
    // Company should also transitively depend on Status through Person
    assert!(company_sig.dependencies.contains(&"Status".to_string()));

    Ok(())
}

#[test]
fn test_circular_dependencies() -> Result<()> {
    let baml = r#"
class User {
  name string
  posts Post[]
}

class Post {
  title string
  author User
}
"#;
    let ir = parse_baml(baml)?;

    // User depends on Post
    let user_sig = get_signature_by_name(&ir, "User")?;
    assert!(user_sig.dependencies.contains(&"Post".to_string()));

    // Post depends on User
    let post_sig = get_signature_by_name(&ir, "Post")?;
    assert!(post_sig.dependencies.contains(&"User".to_string()));

    // Both should handle circular dependency without infinite recursion
    // (test passes if it doesn't hang)

    Ok(())
}

#[test]
fn test_type_alias_dependencies() -> Result<()> {
    let baml = r#"
class Person {
  name string
}

type PersonList = Person[]
type OptionalPerson = Person?
"#;
    let ir = parse_baml(baml)?;

    // PersonList should depend on Person
    let list_sig = get_signature_by_name(&ir, "PersonList")?;
    assert!(list_sig.dependencies.contains(&"Person".to_string()));

    // OptionalPerson should depend on Person
    let optional_sig = get_signature_by_name(&ir, "OptionalPerson")?;
    assert!(optional_sig.dependencies.contains(&"Person".to_string()));

    Ok(())
}

#[test]
fn test_function_dependencies() -> Result<()> {
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
  prompt #"
    Process input
  "#
}
"##;
    let ir = parse_baml(baml)?;

    // Process function should depend on both Input and Output
    let func_sig = get_signature_by_name(&ir, "Process")?;
    assert!(func_sig.dependencies.contains(&"Input".to_string()));
    assert!(func_sig.dependencies.contains(&"Output".to_string()));

    Ok(())
}

#[test]
fn test_union_type_dependencies() -> Result<()> {
    let baml = r#"
class ClassA {
  field string
}

class ClassB {
  field int
}

type UnionType = ClassA | ClassB
"#;
    let ir = parse_baml(baml)?;

    // UnionType should depend on both ClassA and ClassB
    let union_sig = get_signature_by_name(&ir, "UnionType")?;
    assert!(union_sig.dependencies.contains(&"ClassA".to_string()));
    assert!(union_sig.dependencies.contains(&"ClassB".to_string()));

    Ok(())
}

#[test]
fn test_map_type_dependencies() -> Result<()> {
    let baml = r#"
class Value {
  data string
}

type ValueMap = map<string, Value>
"#;
    let ir = parse_baml(baml)?;

    // ValueMap should depend on Value
    let map_sig = get_signature_by_name(&ir, "ValueMap")?;
    assert!(map_sig.dependencies.contains(&"Value".to_string()));

    Ok(())
}

#[test]
fn test_nested_dependencies() -> Result<()> {
    let baml = r#"
enum Level {
  LOW
  HIGH
}

class Config {
  level Level
}

class Settings {
  config Config
}

class Application {
  settings Settings
}
"#;
    let ir = parse_baml(baml)?;

    // Application should depend on Settings, Config, and Level (transitively)
    let app_sig = get_signature_by_name(&ir, "Application")?;
    assert!(app_sig.dependencies.contains(&"Settings".to_string()));
    assert!(app_sig.dependencies.contains(&"Config".to_string()));
    assert!(app_sig.dependencies.contains(&"Level".to_string()));

    Ok(())
}

#[test]
fn test_client_retry_policy_dependency() -> Result<()> {
    let baml = r#"
retry_policy MyRetry {
  max_retries 3
}

client MyClient {
  provider openai
  retry_policy MyRetry
  options {
    model "gpt-4"
  }
}
"#;
    let ir = parse_baml(baml)?;

    // MyClient should depend on MyRetry
    let client_sig = get_signature_by_name(&ir, "MyClient")?;
    assert!(client_sig.dependencies.contains(&"MyRetry".to_string()));

    Ok(())
}

#[test]
fn test_no_dependencies() -> Result<()> {
    let baml = r#"
class SimpleClass {
  field1 string
  field2 int
  field3 bool
}

enum SimpleEnum {
  VALUE1
  VALUE2
}

type SimpleAlias = string
"#;
    let ir = parse_baml(baml)?;

    // All should have no dependencies
    let class_sig = get_signature_by_name(&ir, "SimpleClass")?;
    assert!(class_sig.dependencies.is_empty());

    let enum_sig = get_signature_by_name(&ir, "SimpleEnum")?;
    assert!(enum_sig.dependencies.is_empty());

    let alias_sig = get_signature_by_name(&ir, "SimpleAlias")?;
    assert!(alias_sig.dependencies.is_empty());

    Ok(())
}
