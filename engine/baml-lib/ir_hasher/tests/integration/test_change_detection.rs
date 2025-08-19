use anyhow::Result;

use crate::helpers::*;

// ============================================================================
// Interface Hash Change Tests - These MUST change the interface hash
// ============================================================================

#[test]
fn test_add_class_field_changes_interface() -> Result<()> {
    let baml1 = r#"
class Person {
  name string
}
"#;
    let baml2 = r#"
class Person {
  name string
  age int
}
"#;
    assert_interface_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_remove_class_field_changes_interface() -> Result<()> {
    let baml1 = r#"
class Person {
  name string
  age int
}
"#;
    let baml2 = r#"
class Person {
  name string
}
"#;
    assert_interface_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_rename_class_field_changes_interface() -> Result<()> {
    let baml1 = r#"
class Person {
  name string
}
"#;
    let baml2 = r#"
class Person {
  full_name string
}
"#;
    assert_interface_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_change_field_type_changes_interface() -> Result<()> {
    let baml1 = r#"
class Person {
  age int
}
"#;
    let baml2 = r#"
class Person {
  age string
}
"#;
    assert_interface_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_add_enum_value_changes_interface() -> Result<()> {
    let baml1 = r#"
enum Status {
  ACTIVE
}
"#;
    let baml2 = r#"
enum Status {
  ACTIVE
  INACTIVE
}
"#;
    assert_interface_hash_different(baml1, baml2, "Status")?;
    Ok(())
}

#[test]
fn test_remove_enum_value_changes_interface() -> Result<()> {
    let baml1 = r#"
enum Status {
  ACTIVE
  INACTIVE
}
"#;
    let baml2 = r#"
enum Status {
  ACTIVE
}
"#;
    assert_interface_hash_different(baml1, baml2, "Status")?;
    Ok(())
}

#[test]
fn test_add_assert_constraint_changes_interface() -> Result<()> {
    let baml1 = r#"
type Email = string
"#;
    let baml2 = r#"
type Email = string @assert({{ this|regex_match("^[^@]+@[^@]+$") }})
"#;
    assert_interface_hash_different(baml1, baml2, "Email")?;
    Ok(())
}

#[test]
fn test_add_check_constraint_does_not_affect_hashes() -> Result<()> {
    let baml1 = r#"
class Person {
  age int
}
"#;
    let baml2 = r#"
class Person {
  age int @check(positive, {{ this > 0 }})
}
"#;
    // @check constraints don't affect any hash - they're runtime-only validation
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_implementation_hash_equal(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_change_check_constraint_does_not_affect_hashes() -> Result<()> {
    let baml1 = r#"
type Age = int @check(positive, {{ this > 0 }})
"#;
    let baml2 = r#"
type Age = int @check(positive, {{ this >= 0 }})
"#;
    // @check changes don't affect any hash - they're runtime-only validation
    assert_interface_hash_equal(baml1, baml2, "Age")?;
    assert_implementation_hash_equal(baml1, baml2, "Age")?;
    Ok(())
}

#[test]
fn test_add_dynamic_attribute_changes_interface() -> Result<()> {
    let baml1 = r#"
class Config {
  setting string
}
"#;
    let baml2 = r#"
class Config {
  setting string
  @@dynamic
}
"#;
    assert_interface_hash_different(baml1, baml2, "Config")?;
    Ok(())
}

#[test]
fn test_change_optional_modifier_changes_interface() -> Result<()> {
    let baml1 = r#"
class Person {
  email string
}
"#;
    let baml2 = r#"
class Person {
  email string?
}
"#;
    assert_interface_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_change_function_parameter_type_changes_interface() -> Result<()> {
    let baml1 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process(value: int) -> string {
  client TestClient
  prompt #"Process"#
}
"##;
    let baml2 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process(value: string) -> string {
  client TestClient
  prompt #"Process"#
}
"##;
    assert_interface_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

#[test]
fn test_change_function_return_type_changes_interface() -> Result<()> {
    let baml1 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process() -> string {
  client TestClient
  prompt #"Process"#
}
"##;
    let baml2 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process() -> int {
  client TestClient
  prompt #"Process"#
}
"##;
    assert_interface_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

// ============================================================================
// Implementation-Only Hash Change Tests
// These should change implementation hash BUT NOT interface hash
// ============================================================================

#[test]
fn test_alias_change_only_affects_implementation() -> Result<()> {
    let baml1 = r##"
class Person {
  name string @alias("userName")
}
"##;
    let baml2 = r##"
class Person {
  name string @alias("fullName")
}
"##;
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_implementation_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_description_change_only_affects_implementation() -> Result<()> {
    let baml1 = r##"
enum Status {
  ACTIVE @description("User is active")
}
"##;
    let baml2 = r##"
enum Status {
  ACTIVE @description("Currently active")
}
"##;
    assert_interface_hash_equal(baml1, baml2, "Status")?;
    assert_implementation_hash_different(baml1, baml2, "Status")?;
    Ok(())
}

#[test]
fn test_skip_change_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
class Config {
  api_key string
  public_data string
}
"#;
    let baml2 = r#"
class Config {
  api_key string @skip
  public_data string
}
"#;
    assert_interface_hash_equal(baml1, baml2, "Config")?;
    assert_implementation_hash_different(baml1, baml2, "Config")?;
    Ok(())
}

#[test]
fn test_field_ordering_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
class Person {
  name string
  age int
  email string
}
"#;
    let baml2 = r#"
class Person {
  email string
  name string
  age int
}
"#;
    // Interface should be order-independent (fields sorted by name)
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    // Implementation should be order-sensitive
    assert_implementation_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_enum_value_ordering_affects_implementation() -> Result<()> {
    let baml1 = r#"
enum Priority {
  HIGH
  MEDIUM
  LOW
}
"#;
    let baml2 = r#"
enum Priority {
  LOW
  MEDIUM
  HIGH
}
"#;
    // Enum value order affects implementation
    assert_implementation_hash_different(baml1, baml2, "Priority")?;
    Ok(())
}

#[test]
fn test_function_prompt_change_only_affects_implementation() -> Result<()> {
    let baml1 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Greet(name: string) -> string {
  client TestClient
  prompt #"
    Say hello to {{ name }}
  "#
}
"##;
    let baml2 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Greet(name: string) -> string {
  client TestClient
  prompt #"
    Greet {{ name }} warmly
  "#
}
"##;
    assert_interface_hash_equal(baml1, baml2, "Greet")?;
    assert_implementation_hash_different(baml1, baml2, "Greet")?;
    Ok(())
}

#[test]
fn test_function_client_change_only_affects_implementation() -> Result<()> {
    let baml1 = r##"
client<llm> ClientA {
  provider openai
  options { model "gpt-4" }
}

client<llm> ClientB {
  provider openai
  options { model "gpt-3.5-turbo" }
}

function Process() -> string {
  client ClientA
  prompt #"Process"#
}
"##;
    let baml2 = r##"
client<llm> ClientA {
  provider openai
  options { model "gpt-4" }
}

client<llm> ClientB {
  provider openai
  options { model "gpt-3.5-turbo" }
}

function Process() -> string {
  client ClientB
  prompt #"Process"#
}
"##;
    assert_interface_hash_equal(baml1, baml2, "Process")?;
    assert_implementation_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

// ============================================================================
// Assert Constraint Tests - These MUST change the interface hash
// ============================================================================

#[test]
fn test_add_assert_constraint_to_class_changes_interface() -> Result<()> {
    let baml1 = r#"
class Person {
  age int
}
"#;
    let baml2 = r#"
class Person {
  age int @assert({{ this > 0 }})
}
"#;
    // @assert affects interface hash
    assert_interface_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_change_assert_expression_changes_interface() -> Result<()> {
    let baml1 = r#"
type Age = int @assert({{ this > 0 }})
"#;
    let baml2 = r#"
type Age = int @assert({{ this >= 0 }})
"#;
    // @assert expression changes affect interface
    assert_interface_hash_different(baml1, baml2, "Age")?;
    Ok(())
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_multiple_attributes_interaction() -> Result<()> {
    let baml1 = r##"
class Person {
  first_name string @alias("firstName") @description("First name")
  last_name string @alias("lastName") @description("Last name")
}
"##;
    let baml2 = r##"
class Person {
  first_name string @alias("fname") @description("Given name")
  last_name string @alias("lname") @description("Family name")
}
"##;
    // Both alias and description are implementation-only
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_implementation_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_check_constraint_label_change_does_not_affect_hashes() -> Result<()> {
    let baml1 = r##"
type Age = int @check(positive, {{ this > 0 }})
"##;
    let baml2 = r##"
type Age = int @check(non_negative, {{ this > 0 }})
"##;
    // @check label changes don't affect any hash - they're runtime-only validation
    assert_interface_hash_equal(baml1, baml2, "Age")?;
    assert_implementation_hash_equal(baml1, baml2, "Age")?;
    Ok(())
}

#[test]
fn test_complex_type_structure_changes() -> Result<()> {
    let baml1 = r##"
type Data = string | int
"##;
    let baml2 = r##"
type Data = string | int | bool
"##;
    assert_interface_hash_different(baml1, baml2, "Data")?;
    Ok(())
}

#[test]
fn test_map_type_changes() -> Result<()> {
    let baml1 = r##"
type Dict = map<string, string>
"##;
    let baml2 = r##"
type Dict = map<string, int>
"##;
    assert_interface_hash_different(baml1, baml2, "Dict")?;
    Ok(())
}

#[test]
fn test_list_to_single_type_change() -> Result<()> {
    let baml1 = r#"
class Container {
  items string[]
}
"#;
    let baml2 = r#"
class Container {
  items string
}
"#;
    assert_interface_hash_different(baml1, baml2, "Container")?;
    Ok(())
}

#[test]
fn test_tuple_element_change() -> Result<()> {
    let baml1 = r#"
type Pair = (string, int)
"#;
    let baml2 = r#"
type Pair = (string, string)
"#;
    assert_interface_hash_different(baml1, baml2, "Pair")?;
    Ok(())
}
