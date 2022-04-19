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

#[derive(Clone, Debug, Deserialize)]
struct Config {
    value: Option<String>,
    excluded_variant_gids: Option<Vec<String>>,
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
    const DEFAULT_VALUE: f64 = 50.0;

    let (input, config) = (payload.input, payload.configuration);
    let value: f64 = if let Some(value) = config.value {
        value.parse()?
    } else {
        DEFAULT_VALUE
    };
    let merchandise_lines = &input.merchandise_lines.unwrap_or_default();
    let excluded_variant_gids = &config.excluded_variant_gids.unwrap_or_default();
    let targets = targets(&merchandise_lines, &excluded_variant_gids);
    return Ok(build_output(value, targets));
}

fn targets(
    merchandise_lines: &Vec<input::InputMerchandiseLines>,
    excluded_variant_gids: &[String],
) -> Vec<output::ProductVariantTarget> {
    let excluded_variant_ids: Vec<u64> = excluded_variant_gids
        .iter()
        .filter_map(|gid| gid.split('/').last().map(|id| id.parse().unwrap()))
        .collect();

    merchandise_lines
        .iter()
        .filter_map(|line| match line.variant {
            Some(ref variant) if !excluded_variant_ids.contains(&(variant.id as u64)) => {
                Some(output::ProductVariantTarget {
                    target_type: output::TargetType::ProductVariant,
                    id: variant.id,
                    quantity: None,
                })
            }
            _ => None,
        })
        .collect()
}

fn build_output(value: f64, targets: Vec<output::ProductVariantTarget>) -> output::Variables {
    output::Variables {
        discounts: vec![output::Discount {
            message: Some(format!("{}% off", value)),
            conditions: None,
            targets,
            value: output::Value {
                type_: output::ValueType::Percentage,
                value,
            },
        }],
        discount_application_strategy: output::DiscountApplicationStrategy::First,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(configuration: Config) -> Payload {
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
                "excluded_variant_gids": null,
            }
        }
        "#;
        let default_payload: Payload = serde_json::from_str(&input).unwrap();
        Payload {
            configuration,
            ..default_payload
        }
    }

    #[test]
    fn test_discount_with_default_value() {
        let payload = payload(Config {
            value: None,
            excluded_variant_gids: None,
        });
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "50% off",
                    "conditions": null,
                    "targets": [
                        { "target_type": "product_variant", "id": 0, "quantity": null },
                        { "target_type": "product_variant", "id": 1, "quantity": null }
                    ],
                    "value": {
                        "type": "percentage",
                        "value": 50.0
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
        let payload = payload(Config {
            value: Some("10".to_string()),
            excluded_variant_gids: None,
        });
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "10% off",
                    "conditions": null,
                    "targets": [
                        { "target_type": "product_variant", "id": 0, "quantity": null },
                        { "target_type": "product_variant", "id": 1, "quantity": null }
                    ],
                    "value": {
                        "type": "percentage",
                        "value": 10.0
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
        let payload = payload(Config {
            value: None,
            excluded_variant_gids: Some(vec!["gid://shopify/ProductVariant/0".to_string()]),
        });
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "50% off",
                    "conditions": null,
                    "targets": [
                        { "target_type": "product_variant", "id": 1, "quantity": null }
                    ],
                    "value": {
                        "type": "percentage",
                        "value": 50.0
                    }
                }],
                "discount_application_strategy": "first"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
