use anyhow::Result;

use crate::helpers::*;

#[test]
fn test_no_arg_function() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function GetGreeting() -> string {
  client TestClient
  prompt #"Say hello"#
}
"##;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "GetGreeting")?;
    assert_ne!(hash, 0, "No-arg function should have non-zero hash");
    Ok(())
}

#[test]
fn test_single_param_function() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Echo(input: string) -> string {
  client TestClient
  prompt #"Echo: {{ input }}"#
}
"##;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Echo")?;
    assert_ne!(hash, 0, "Single-param function should have non-zero hash");
    Ok(())
}

#[test]
fn test_multi_param_function() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process(name: string, age: int, active: bool?) -> string {
  client TestClient
  prompt #"Process user: {{ name }}, {{ age }}, {{ active }}"#
}
"##;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "Process")?;
    assert_ne!(hash, 0, "Multi-param function should have non-zero hash");
    Ok(())
}

#[test]
fn test_function_param_type_change_affects_interface() -> Result<()> {
    let baml1 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process(age: int) -> string {
  client TestClient
  prompt #"
    Age: {{ age }}
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

function Process(age: string) -> string {
  client TestClient
  prompt #"
    Age: {{ age }}
  "#
}
"##;
    assert_interface_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

#[test]
fn test_function_param_name_change_affects_interface() -> Result<()> {
    let baml1 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process(name: string) -> string {
  client TestClient
  prompt #"
    Name: {{ name }}
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

function Process(full_name: string) -> string {
  client TestClient
  prompt #"
    Name: {{ full_name }}
  "#
}
"##;
    assert_interface_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

#[test]
fn test_function_return_type_change_affects_interface() -> Result<()> {
    let baml1 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process() -> string {
  client TestClient
  prompt #"
    Hello
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

function Process() -> int {
  client TestClient
  prompt #"
    Hello
  "#
}
"##;
    assert_interface_hash_different(baml1, baml2, "Process")?;
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

function Process(input: string) -> string {
  client TestClient
  prompt #"
    Process this: {{ input }}
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

function Process(input: string) -> string {
  client TestClient
  prompt #"
    Handle the input: {{ input }}
  "#
}
"##;
    assert_interface_hash_equal(baml1, baml2, "Process")?;
    assert_implementation_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

#[test]
fn test_function_client_change_only_affects_implementation() -> Result<()> {
    let baml1 = r##"
client<llm> ClientA {
  provider openai
  options {
    model "gpt-4"
  }
}

client<llm> ClientB {
  provider openai
  options {
    model "gpt-3.5-turbo"
  }
}

function Process(input: string) -> string {
  client ClientA
  prompt #"
    Process: {{ input }}
  "#
}
"##;
    let baml2 = r##"
client<llm> ClientA {
  provider openai
  options {
    model "gpt-4"
  }
}

client<llm> ClientB {
  provider openai
  options {
    model "gpt-3.5-turbo"
  }
}

function Process(input: string) -> string {
  client ClientB
  prompt #"
    Process: {{ input }}
  "#
}
"##;
    assert_interface_hash_equal(baml1, baml2, "Process")?;
    assert_implementation_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

#[test]
fn test_function_with_complex_types() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

class User {
  name string
  age int
}

class Result {
  users User[]
  total int
}

function ProcessUsers(users: User[]) -> Result {
  client TestClient
  prompt #"
    Process users
  "#
}
"##;
    let ir = parse_baml(baml)?;
    let hash = get_interface_hash(&ir, "ProcessUsers")?;
    assert_ne!(
        hash, 0,
        "Function with complex types should have non-zero hash"
    );
    Ok(())
}

#[test]
fn test_function_with_assert_constraints() -> Result<()> {
    let baml1 = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process(age: int) -> string {
  client TestClient
  prompt #"
    Age: {{ age }}
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

function Process(age: int) -> string @assert({{ this | length > 0 }}) {
  client TestClient
  prompt #"
    Age: {{ age }}
  "#
}
"##;
    // @assert constraints on return types affect interface hash
    assert_interface_hash_different(baml1, baml2, "Process")?;
    Ok(())
}

#[test]
fn test_function_hash_stability() -> Result<()> {
    let baml = r##"
client<llm> TestClient {
  provider openai
  options {
    model gpt-4
  }
}

function Process(name: string, age: int) -> string {
  client TestClient
  prompt #"Process user"#
}
"##;
    let ir = parse_baml(baml)?;
    assert_hash_stable(&ir)?;
    Ok(())
}
