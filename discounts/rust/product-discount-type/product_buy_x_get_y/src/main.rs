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
    buy_variant_gids: Vec<String>,
    buy_quantity: Option<ID>,
    get_variant_gid: String,
    get_quantity: Option<ID>,
}

impl Config {
    const DEFAULT_BUY_X: ID = 4;
    const DEFAULT_GET_Y: ID = 2;

    fn buy_variant_ids(&self) -> Vec<ID> {
        self.buy_variant_gids
            .iter()
            .map(Self::convert_gid_to_id)
            .collect()
    }

    fn get_variant_id(&self) -> ID {
        Self::convert_gid_to_id(&self.get_variant_gid)
    }

    fn buy_x(&self) -> ID {
        self.buy_quantity.unwrap_or(Self::DEFAULT_BUY_X)
    }

    fn get_y(&self) -> ID {
        self.get_quantity.unwrap_or(Self::DEFAULT_GET_Y)
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
    let (input, config) = (payload.input, payload.configuration);
    let merchandise_lines = &input.merchandise_lines.unwrap_or_default();

    let conditions = vec![output::ProductCondition {
        targetType: output::TargetType::ProductVariant,
        ids: config.buy_variant_ids(),
        minimumAmount: None,
        minimumQuantity: Some(config.buy_x()),
    }];

    let targets = merchandise_lines
        .iter()
        .filter_map(|line| match line.variant {
            Some(ref variant) if (variant.id as ID == config.get_variant_id()) => {
                Some(output::ProductVariantTarget {
                    targetType: output::TargetType::ProductVariant,
                    id: variant.id,
                    // hard code
                    quantity: Some(config.get_y()),
                })
            }
            _ => None,
        })
        .collect();

    Ok(output::Variables {
        discounts: vec![output::Discount {
            message: Some(format!("Buy {} get {}", config.buy_x(), config.get_y())),
            conditions: Some(conditions),
            targets,
            value: output::Value {
                type_: output::ValueType::Percentage,
                value: 100.0,
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
                    { "variant": { "id": 1 } },
                    { "variant": { "id": 2 } }
                ]
            },
            "configuration": {
                "buy_variant_gids": ["gid://shopify/ProductVariant/1", "gid://shopify/ProductVariant/2"],
                "buy_quantity": null,
                "get_variant_gid": "gid://shopify/ProductVariant/1",
                "get_quantity": null
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
                    "message": "Buy 4 get 2",
                    "conditions": [
                        { "targetType": "product_variant", "ids": [1, 2], "minimumAmount": null, "minimumQuantity": 4 }
                    ],
                    "targets": [
                        { "targetType": "product_variant", "id": 1, "quantity": 2 }
                    ],
                    "value": {
                        "type": "PERCENTAGE",
                        "value": 100.0,
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
            buy_variant_gids: vec![
                "gid://shopify/ProductVariant/1".to_string(),
                "gid://shopify/ProductVariant/2".to_string(),
                "gid://shopify/ProductVariant/3".to_string(),
            ],
            buy_quantity: Some(2),
            get_variant_gid: "gid://shopify/ProductVariant/2".to_string(),
            get_quantity: Some(1),
        }));
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "Buy 2 get 1",
                    "conditions": [
                        { "targetType": "product_variant", "ids": [1, 2, 3], "minimumAmount": null, "minimumQuantity": 2 }
                    ],
                    "targets": [
                        { "targetType": "product_variant", "id": 2, "quantity": 1 }
                    ],
                    "value": {
                        "type": "PERCENTAGE",
                        "value": 100.0,
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
    fn test_discount_with_no_match_variants() {
        let payload = payload(Some(Config {
            buy_variant_gids: vec!["gid://shopify/ProductVariant/1".to_string()],
            buy_quantity: Some(2),
            get_variant_gid: "gid://shopify/ProductVariant/5".to_string(),
            get_quantity: Some(1),
        }));
        let output = serde_json::json!(script(payload).unwrap());

        let expected_json = r#"
            {
                "discounts": [{
                    "message": "Buy 2 get 1",
                    "conditions": [
                        { "targetType": "product_variant", "ids": [1], "minimumAmount": null, "minimumQuantity": 2 }
                    ],
                    "targets": [],
                    "value": {
                        "type": "PERCENTAGE",
                        "value": 100.0,
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
