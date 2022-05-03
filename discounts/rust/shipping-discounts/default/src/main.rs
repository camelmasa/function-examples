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
}

impl Config {
    const DEFAULT_VALUE: f64 = 20.0;

    fn get_value(&self) -> f64 {
        match &self.value {
            Some(value) => value.parse().unwrap(),
            _ => Self::DEFAULT_VALUE,
        }
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
    let (input, config) = (payload.input, payload.configuration);
    let delivery_lines = &input.delivery_lines.unwrap_or_default();

    let targets = delivery_lines
        .iter()
        .filter_map(|delivery_line| {
            if let Some(index) = delivery_line.index {
                Some(output::Target {
                    targetType: output::TargetType::ShippingLine,
                    index,
                })
            } else {
                None
            }
        })
        .collect();
    Ok(output::Variables {
        discounts: vec![output::Discount {
            message: Some(format!("{}% off shipping", config.get_value())),
            conditions: None,
            targets,
            value: output::Value {
                type_: output::ValueType::Percentage,
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
                "deliveryLines": [
                    { "index": 0 },
                    { "index": 1 }
                ]
            },
            "configuration": {
                "value": null
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
                    "message": "20% off shipping",
                    "conditions": null,
                    "targets": [
                        {
                            "targetType": "SHIPPING_LINE",
                            "index": 0
                        },
                        {
                            "targetType": "SHIPPING_LINE",
                            "index": 1
                        }
                    ],
                    "value": {
                        "type": "PERCENTAGE",
                        "value": 20.0,
                        "appliesToEachItem": null
                    }
                }],
                "discount_application_strategy": "FIRST"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }

    #[test]
    fn test_discount_with_value() {
        let payload = payload(Some(Config {
            value: Some("10.0".to_string()),
        }));
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "10% off shipping",
                    "conditions": null,
                    "targets": [
                        {
                            "targetType": "SHIPPING_LINE",
                            "index": 0
                        },
                        {
                            "targetType": "SHIPPING_LINE",
                            "index": 1
                        }
                    ],
                    "value": {
                        "type": "PERCENTAGE",
                        "value": 10.0,
                        "appliesToEachItem": null
                    }
                }],
                "discount_application_strategy": "FIRST"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
