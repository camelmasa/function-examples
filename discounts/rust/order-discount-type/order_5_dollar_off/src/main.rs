use serde::{Deserialize, Serialize};

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "input.graphql",
    response_derives = "Debug, Clone, PartialEq",
    normalization = "rust"
)]
struct Input;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "output.graphql",
    response_derives = "Debug, Clone, PartialEq",
    normalization = "rust"
)]
struct Output;

type ID = i64;

#[derive(Clone, Debug, Deserialize)]
struct Config {
    value: Option<String>,
    excluded_variant_gids: Option<Vec<String>>,
}

impl Config {
    const DEFAULT_VALUE: f64 = 5.0;

    fn get_value(&self) -> f64 {
        match &self.value {
            Some(value) => value.parse().unwrap(),
            _ => Self::DEFAULT_VALUE,
        }
    }

    fn excluded_variant_ids(&self) -> Vec<ID> {
        match &self.excluded_variant_gids {
            Some(excluded_variant_gids) => excluded_variant_gids
                .iter()
                .map(Self::convert_gid_to_id)
                .collect(),
            _ => {
                vec![]
            }
        }
    }

    fn convert_gid_to_id(gid: &String) -> ID {
        gid.split('/').last().map(|id| id.parse().unwrap()).unwrap()
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Payload {
    input: input::ResponseData,
    configuration: Config,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payload: Payload = serde_json::from_reader(std::io::BufReader::new(std::io::stdin()))?;
    let mut out = std::io::stdout();
    let mut serializer = serde_json::Serializer::new(&mut out);
    script(payload)?.serialize(&mut serializer)?;
    Ok(())
}

fn script(payload: Payload) -> Result<output::Variables, Box<dyn std::error::Error>> {
    let (_input, config) = (payload.input, payload.configuration);
    // let merchandise_lines = &input.merchandise_lines.unwrap_or_default();

    let targets = vec![output::Target {
        targetType: output::TargetType::OrderSubtotal,
        excludedVariantIds: Some(config.excluded_variant_ids()),
        id: None,
        quantity: None,
    }];
    Ok(output::Variables {
        discounts: vec![output::Discount {
            message: Some(format!("${} off order subtotal", config.get_value())),
            conditions: None,
            targets,
            value: output::Value {
                type_: output::ValueType::FixedAmount,
                value: config.get_value(),
                appliesToEachItem: None,
            },
        }],
        discount_application_strategy: output::DiscountApplicationStrategy::First,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(config: Option<Config>) -> Payload {
        let input = r#"
        {
            "input": {
                "merchandiseLines": [
                    { "variant": { "id": 0 } },
                    { "variant": { "id": 1 } }
                ]
            },
            "configuration": {
                "value": null,
                "excluded_variant_gids": ["gid://shopify/ProductVariant/1", "gid://shopify/ProductVariant/2"]
            }
        }
        "#;
        let default_payload: Payload = serde_json::from_str(&input).unwrap();

        match config {
            Some(configuration) => Payload {
                configuration,
                ..default_payload
            },
            None => default_payload,
        }
    }

    #[test]
    fn test_discount_with_default_value() {
        let payload = payload(None);
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "$5 off order subtotal",
                    "conditions": null,
                    "targets": [{
                        "targetType": "ORDER_SUBTOTAL",
                        "excludedVariantIds": [1, 2],
                        "id": null,
                        "quantity": null
                    }],
                    "value": {
                        "type": "FIXED_AMOUNT",
                        "value": 5.0,
                        "appliesToEachItem": null
                    }
                }],
                "discount_application_strategy": "first"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }

    #[test]
    fn test_discount_with_value() {
        let payload = payload(Some(Config {
            value: Some("10.0".to_string()),
            excluded_variant_gids: None,
        }));
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "$10 off order subtotal",
                    "conditions": null,
                    "targets": [{
                        "targetType": "ORDER_SUBTOTAL",
                        "excludedVariantIds": [],
                        "id": null,
                        "quantity": null
                    }],
                    "value": {
                        "type": "FIXED_AMOUNT",
                        "value": 10.0,
                        "appliesToEachItem": null
                    }
                }],
                "discount_application_strategy": "first"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }

    #[test]
    fn test_discount_with_excluded_variant_gids() {
        let payload = payload(Some(Config {
            value: None,
            excluded_variant_gids: Some(vec!["gid://shopify/ProductVariant/0".to_string()]),
        }));
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "$5 off order subtotal",
                    "conditions": null,
                    "targets": [{
                        "targetType": "ORDER_SUBTOTAL",
                        "excludedVariantIds": [0],
                        "id": null,
                        "quantity": null
                    }],
                    "value": {
                        "type": "FIXED_AMOUNT",
                        "value": 5.0,
                        "appliesToEachItem": null
                    }
                }],
                "discount_application_strategy": "first"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
