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
{
  "buy_variant_gids": [
    "gid://shopify/ProductVariant/1",
    "gid://shopify/ProductVariant/2"
  ],
  "get_variant_gid": "gid://shopify/ProductVariant/1"
}
```

output

```JSON
{
  "discounts": [
    {
      "targets": [
        {
          "targetType": "product_variant",
          "id": 1,
          "quantity": 2
        }
      ],
      "value": {
        "type": "PERCENTAGE",
        "value": 100.0,
        "appliesToEachItem": null
      },
      "conditions": [
        {
          "targetType": "product_variant",
          "ids": [
            1,
            2
          ],
          "minimumAmount": null,
          "minimumQuantity": 4
        }
      ],
      "message": "Buy 4 get 2"
    }
  ],
  "discount_application_strategy": "first"
}
```


