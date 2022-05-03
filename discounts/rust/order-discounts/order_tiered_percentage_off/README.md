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
{"tiers":[{"amount":300,"percentage":50},{"amount":200,"percentage":20},{"amount":100,"percentage":10}]}
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
          "excludedVariantIds": null
        }
      ],
      "value": {
        "type": "PERCENTAGE",
        "value": 50.0,
        "appliesToEachItem": null
      },
      "conditions": [
        {
          "targetType": "ORDER_SUBTOTAL",
          "ids": null,
          "excludedVariantIds": null,
          "minimumAmount": 30000.0,
          "minimumQuantity": null
        }
      ],
      "message": "Spend $300 Get 50% off"
    },
    {
      "targets": [
        {
          "targetType": "ORDER_SUBTOTAL",
          "id": null,
          "quantity": null,
          "excludedVariantIds": null
        }
      ],
      "value": {
        "type": "PERCENTAGE",
        "value": 20.0,
        "appliesToEachItem": null
      },
      "conditions": [
        {
          "targetType": "ORDER_SUBTOTAL",
          "ids": null,
          "excludedVariantIds": null,
          "minimumAmount": 20000.0,
          "minimumQuantity": null
        }
      ],
      "message": "Spend $200 Get 20% off"
    },
    {
      "targets": [
        {
          "targetType": "ORDER_SUBTOTAL",
          "id": null,
          "quantity": null,
          "excludedVariantIds": null
        }
      ],
      "value": {
        "type": "PERCENTAGE",
        "value": 10.0,
        "appliesToEachItem": null
      },
      "conditions": [
        {
          "targetType": "ORDER_SUBTOTAL",
          "ids": null,
          "excludedVariantIds": null,
          "minimumAmount": 10000.0,
          "minimumQuantity": null
        }
      ],
      "message": "Spend $100 Get 10% off"
    }
  ],
  "discount_application_strategy": "FIRST"
}
```


