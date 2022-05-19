### Prerequisites

* [Install Rust](https://www.rust-lang.org/tools/install)
* Add `wasm32-wasi` target: `rustup target add wasm32-wasi`
* Install `wasm-opt`: `brew install binaryen`

### Build

```
make
```

### Note

If you have to interact with the auto-generated Rust types from GQL schema,

  * Install [graphql_client_cli](https://github.com/graphql-rust/graphql-client/tree/main/graphql_client_cli) by `cargo install graphql_client_cli`
  * `graphql-client generate -o build/ ./output.graphql --schema-path ./schema.graphql` (using `output.graphql` as an example)
  * Check `build/output.rs`

Use SCREAMING_SNAKE_CASE as the conventional enum value. However, the codegen tool (graphql_client)
will generate the type as `ValueType::FixedAmount`.

```
enum ValueType {
  FIXED_AMOUNT
}
```

### Demo output

```JSON
{
  "result": {
    "discounts": [
      {
        "targets": [
          {
            "id": "gid://shopify/ProductVariant/1",
            "quantity": null
          }
        ],
        "value": {
          "value": 5.0,
          "appliesToEachItem": null
        },
        "conditions": null,
        "message": "$5 off"
      }
    ],
    "discount_application_strategy": "FIRST"
  }
}
```


