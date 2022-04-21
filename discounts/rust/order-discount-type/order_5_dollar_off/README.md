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

### Demo

configuration

```json
{"value":"5.0","excluded_variant_gids":[]}
```

output

```JSON
{
  "discounts": [
    {
      "targets": [
        {
          "targetType": "ORDER_SUBTOTAL",
          "id": null,
          "quantity": null,
          "excludedVariantIds": [
          ]
        }
      ],
      "value": {
        "type": "FIXED_AMOUNT",
        "value": 5.0,
        "appliesToEachItem": null
      },
      "conditions": null,
      "message": "$5 off order subtotal"
    }
  ],
  "discount_application_strategy": "first"
}
```


