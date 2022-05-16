use serde::{Deserialize, Serialize};

use graphql_client::GraphQLQuery;

type UnsignedInt64 = u64;
type Void = ();

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "output.graphql",
    response_derives = "Debug, Clone, PartialEq",
    normalization = "rust"
)]
struct Output;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Configuration {
    value: Option<String>,
    excluded_variant_gids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
struct Input {
    configuration: Configuration,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: Input = serde_json::from_reader(std::io::BufReader::new(std::io::stdin()))?;
    let mut out = std::io::stdout();
    let mut serializer = serde_json::Serializer::new(&mut out);
    script(input)?.serialize(&mut serializer)?;
    Ok(())
}

fn script(input: Input) -> Result<output::ScriptOutput, Box<dyn std::error::Error>> {
    const DEFAULT_VALUE: f64 = 50.0;

    let config = input.configuration;
    let value: f64 = if let Some(value) = config.value {
        value.parse()?
    } else {
        DEFAULT_VALUE
    };
    let excluded_variant_gids = &config.excluded_variant_gids.unwrap_or_default();
    let targets = vec![target(excluded_variant_gids)];
    Ok(build_output(value, targets))
}

fn target(excluded_variant_gids: &[String]) -> output::Target {
    output::Target {
        orderSubtotal: Some(output::OrderSubtotalTarget {
            excludedVariantIds: excluded_variant_gids
                .iter()
                .filter_map(|gid| gid.split('/').last().map(|id| id.parse().unwrap()))
                .collect(),
        }),
        productVariant: None,
    }
}

fn build_output(value: f64, targets: Vec<output::Target>) -> output::ScriptOutput {
    output::ScriptOutput {
        discounts: vec![output::Discount {
            message: Some(format!("{}% off", value)),
            conditions: None,
            targets,
            value: output::Value {
                percentage: Some(output::Percentage { value }),
                fixedAmount: None,
            },
        }],
        discountApplicationStrategy: output::DiscountApplicationStrategy::First,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discount_with_default_value() {
        let input = Input {
            configuration: Configuration {
                value: None,
                excluded_variant_gids: None,
            },
        };
        let output = serde_json::json!(script(input).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "50% off",
                    "conditions": null,
                    "targets": [{
                        "orderSubtotal": { "excludedVariantIds": [] },
                        "productVariant": null
                    }],
                    "value": {
                        "percentage": { "value": 50.0 },
                        "fixedAmount": null
                    }
                }],
                "discountApplicationStrategy": "FIRST"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }

    #[test]
    fn test_discount_with_value() {
        let input = Input {
            configuration: Configuration {
                value: Some("10".to_string()),
                excluded_variant_gids: None,
            },
        };
        let output = serde_json::json!(script(input).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "10% off",
                    "conditions": null,
                    "targets": [{
                        "orderSubtotal": { "excludedVariantIds": [] },
                        "productVariant": null
                    }],
                    "value": {
                        "percentage": { "value": 10.0 },
                        "fixedAmount": null
                    }
                }],
                "discountApplicationStrategy": "FIRST"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }

    #[test]
    fn test_discount_with_excluded_variant_gids() {
        let input = Input {
            configuration: Configuration {
                value: None,
                excluded_variant_gids: Some(vec!["gid://shopify/ProductVariant/0".to_string()]),
            },
        };
        let output = serde_json::json!(script(input).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "50% off",
                    "conditions": null,
                    "targets": [{
                        "orderSubtotal": { "excludedVariantIds": [0] },
                        "productVariant": null
                    }],
                    "value": {
                        "percentage": { "value": 50.0 },
                        "fixedAmount": null
                    }
                }],
                "discountApplicationStrategy": "FIRST"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
