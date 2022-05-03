### Prerequisites

* [Install Rust](https://www.rust-lang.org/tools/install)
* Add `wasm32-wasi` target: `rustup target add wasm32-wasi`

### Build

```
make
```

### Note

If you have to interact with the auto-generated Rust types from GQL schema,

  * Install [graphql_client_cli](https://github.com/graphql-rust/graphql-client/tree/main/graphql_client_cli) by `cargo install graphql_client_cli`
  * `graphql-client generate -o build/ ./input.graphql --schema-path ./schema.graphql` (using `input.graphql` as an example)
  * Check `build.input.rs`

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
  "discounts": [
    {
      "targets": [
        {
          "targetType": "PRODUCT_VARIANT",
          "id": 2,
          "quantity": null
        },
        {
          "targetType": "PRODUCT_VARIANT",
          "id": 1,
          "quantity": null
        }
      ],
      "value": {
        "type": "FIXED_AMOUNT",
        "value": 5.0,
        "appliesToEachItem": true
      },
      "conditions": null,
      "message": "$5 off"
    }
  ],
  "discount_application_strategy": "FIRST"
}

```


