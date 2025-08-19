use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_empty_class() -> Result<()> {
    let baml = r#"
class Empty {
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Empty")?;
    assert_ne!(hash, 0, "Empty class should have non-zero hash");
    Ok(())
}

#[test]
fn test_class_with_single_field() -> Result<()> {
    let baml = r#"
class Person {
  name string
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Person")?;
    assert_ne!(hash, 0, "Class with single field should have non-zero hash");
    Ok(())
}

#[test]
fn test_class_with_multiple_fields() -> Result<()> {
    let baml = r#"
class Person {
  name string
  age int
  email string?
}
"#;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Person")?;
    assert_ne!(
        hash, 0,
        "Class with multiple fields should have non-zero hash"
    );
    Ok(())
}

#[test]
fn test_class_field_type_change_affects_interface() -> Result<()> {
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
fn test_class_field_name_change_affects_interface() -> Result<()> {
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
fn test_class_field_optional_change_affects_interface() -> Result<()> {
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
fn test_class_field_alias_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
class Person {
  name string @alias("userName")
}
"#;
    let baml2 = r#"
class Person {
  name string @alias("fullName")
}
"#;
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_implementation_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_class_field_description_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
class Person {
  name string @description("User's name")
}
"#;
    let baml2 = r#"
class Person {
  name string @description("The person's full name")
}
"#;
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_implementation_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_class_field_skip_only_affects_implementation() -> Result<()> {
    let baml1 = r#"
class Person {
  secret string
}
"#;
    let baml2 = r#"
class Person {
  secret string @skip
}
"#;
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_implementation_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_class_field_ordering_affects_implementation_not_interface() -> Result<()> {
    let baml1 = r#"
class Person {
  name string
  age int
}
"#;
    let baml2 = r#"
class Person {
  age int
  name string
}
"#;
    // Interface hash should be the same (fields sorted by name)
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    // Implementation hash should be different (fields in original order)
    assert_implementation_hash_different(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_class_with_check_constraints_does_not_affect_hashes() -> Result<()> {
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
fn test_class_check_constraint_expression_change_does_not_affect_hashes() -> Result<()> {
    let baml1 = r#"
class Person {
  age int @check(positive, {{ this > 0 }})
}
"#;
    let baml2 = r#"
class Person {
  age int @check(positive, {{ this >= 0 }})
}
"#;
    // @check expression changes don't affect any hash - they're runtime-only validation
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_implementation_hash_equal(baml1, baml2, "Person")?;
    Ok(())
}

#[test]
fn test_class_hash_stability() -> Result<()> {
    let baml = r#"
class Person {
  name string
  age int
  email string?
}
"#;
    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;
    Ok(())
}
