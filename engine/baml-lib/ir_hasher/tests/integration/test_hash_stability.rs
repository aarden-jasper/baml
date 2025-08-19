use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_hash_determinism() -> Result<()> {
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
  email string?
}

enum Status {
  ACTIVE
  INACTIVE
  PENDING
}

function ProcessPerson(person: Person) -> Status {
  client TestClient
  prompt #"
    Determine status for person
  "#
}
"##;

    // Parse the same BAML multiple times
    let ir1 = parse_baml(baml)?;
    let ir2 = parse_baml(baml)?;
    let ir3 = parse_baml(baml)?;

    // Get hashes for each parse
    let hash1_person = get_interface_hash(&ir1, "Person")?;
    let hash2_person = get_interface_hash(&ir2, "Person")?;
    let hash3_person = get_interface_hash(&ir3, "Person")?;

    // All hashes should be identical
    assert_eq!(hash1_person, hash2_person, "Hash should be deterministic");
    assert_eq!(hash2_person, hash3_person, "Hash should be deterministic");

    // Test for other types too
    let hash1_status = get_interface_hash(&ir1, "Status")?;
    let hash2_status = get_interface_hash(&ir2, "Status")?;
    let hash3_status = get_interface_hash(&ir3, "Status")?;

    assert_eq!(hash1_status, hash2_status);
    assert_eq!(hash2_status, hash3_status);

    Ok(())
}

#[test]
fn test_whitespace_insensitivity() -> Result<()> {
    let baml1 = r#"
class Person {
  name string
  age int
}
"#;

    let baml2 = r#"
class   Person   {
    name    string
    age     int
}
"#;

    let baml3 = r#"
class Person{
name string
age int
}
"#;

    // All variations should produce the same hash
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_interface_hash_equal(baml2, baml3, "Person")?;
    assert_interface_hash_equal(baml1, baml3, "Person")?;

    Ok(())
}

#[test]
fn test_comment_insensitivity() -> Result<()> {
    let baml1 = r#"
class Person {
  name string
  age int
}
"#;

    let baml2 = r#"
// This is a person class
class Person {
  name string  // The person's name
  age int      // The person's age
}
"#;

    let baml3 = r#"
class Person { // Represents a person
  // Personal information
  name string
  age int
  // End of class
}
"#;

    // Comments should not affect hash
    assert_interface_hash_equal(baml1, baml2, "Person")?;
    assert_interface_hash_equal(baml2, baml3, "Person")?;
    assert_interface_hash_equal(baml1, baml3, "Person")?;

    Ok(())
}

#[test]
fn test_hash_stability_across_types() -> Result<()> {
    let baml = r##"
class MyClass {
  field string
}

enum MyEnum {
  VALUE
}

function MyFunction() -> string {
  prompt #"
    Hello
  "#
}

type MyAlias = string

client MyClient {
  provider openai
  options {
    model "gpt-4"
  }
}

retry_policy MyRetry {
  max_retries 3
}
"##;

    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;

    Ok(())
}

#[test]
fn test_complex_type_hash_stability() -> Result<()> {
    let baml = r#"
class Address {
  street string
  city string
  country string
}

class Person {
  name string
  addresses Address[]
  primary_address Address?
  metadata map<string, string>
}

type PersonOrAddress = Person | Address
type PersonList = Person[]
type PersonMap = map<string, Person>
"#;

    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;

    Ok(())
}

#[test]
fn test_hash_stability_with_constraints() -> Result<()> {
    let baml = r#"
type PositiveInt = int @check(positive, {{ this > 0 }})
type Email = string @assert({{ this|regex_match("^[^@]+@[^@]+$") }})

class ValidatedData {
  age PositiveInt
  email Email
  score int @check(range, {{ this >= 0 and this <= 100 }})
}
"#;

    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;

    Ok(())
}

#[test]
fn test_hash_stability_with_circular_deps() -> Result<()> {
    let baml = r#"
class User {
  name string
  posts Post[]
  comments Comment[]
}

class Post {
  title string
  author User
  comments Comment[]
}

class Comment {
  text string
  author User
  post Post
}
"#;

    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;

    Ok(())
}

#[test]
fn test_empty_types_hash_stability() -> Result<()> {
    let baml = r#"
class EmptyClass {
}

enum EmptyEnum {
}
"#;

    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;

    Ok(())
}
