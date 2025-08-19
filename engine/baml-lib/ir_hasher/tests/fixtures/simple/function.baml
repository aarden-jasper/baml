function SimpleFunction(input: string) -> string {
  prompt #"
    Echo this: {{ input }}
  "#
}

function NoArgFunction() -> string {
  prompt #"
    Say hello
  "#
}

function MultiParamFunction(name: string, age: int, active: bool?) -> string {
  prompt #"
    Process user: {{ name }}, age: {{ age }}, active: {{ active }}
  "#
}

client DummyClient {
  provider openai
  options {
    model "gpt-4"
  }
}

function WithClient(input: string) -> string {
  client DummyClient
  prompt #"
    Process: {{ input }}
  "#
}