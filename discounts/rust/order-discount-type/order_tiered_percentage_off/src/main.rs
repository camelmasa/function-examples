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
struct Tier {
    amount: f64,
    percentage: f64,
}
impl Tier {
    fn amount_in_cents(&self) -> f64 {
        self.amount * 100.0
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Config {
    tiers: Vec<Tier>,
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

    let discounts: Vec<output::Discount> = config
        .tiers
        .iter()
        .map(|tier| output::Discount {
            message: Some(format!(
                "Spend ${} Get {}% off",
                tier.amount, tier.percentage
            )),
            conditions: Some(vec![output::Condition {
                targetType: output::TargetType::OrderSubtotal,
                ids: None,
                excludedVariantIds: None,
                minimumAmount: Some(tier.amount_in_cents()),
                minimumQuantity: None,
            }]),
            targets: vec![output::Target {
                targetType: output::TargetType::OrderSubtotal,
                excludedVariantIds: None,
                id: None,
                quantity: None,
            }],
            value: output::Value {
                type_: output::ValueType::Percentage,
                value: tier.percentage,
                appliesToEachItem: None,
            },
        })
        .collect();

    Ok(output::Variables {
        discounts: discounts,
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
                "tiers": [
                    { "amount": 300, "percentage": 50 },
                    { "amount": 200, "percentage": 20 },
                    { "amount": 100, "percentage": 10 }
                ]
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
                "discounts": [
                    {
                        "message": "Spend $300 Get 50% off",
                        "conditions": [{
                            "targetType": "ORDER_SUBTOTAL",
                            "ids": null,
                            "excludedVariantIds": null,
                            "minimumAmount": 30000.0,
                            "minimumQuantity": null
                        }],
                        "targets": [{
                            "targetType": "ORDER_SUBTOTAL",
                            "excludedVariantIds": null,
                            "id": null,
                            "quantity": null
                        }],
                        "value": {
                            "type": "PERCENTAGE",
                            "value": 50.0,
                            "appliesToEachItem": null
                        }
                    },
                    {
                        "message": "Spend $200 Get 20% off",
                        "conditions": [{
                            "targetType": "ORDER_SUBTOTAL",
                            "ids": null,
                            "excludedVariantIds": null,
                            "minimumAmount": 20000.0,
                            "minimumQuantity": null
                        }],
                        "targets": [{
                            "targetType": "ORDER_SUBTOTAL",
                            "excludedVariantIds": null,
                            "id": null,
                            "quantity": null
                        }],
                        "value": {
                            "type": "PERCENTAGE",
                            "value": 20.0,
                            "appliesToEachItem": null
                        }
                    },
                    {
                        "message": "Spend $100 Get 10% off",
                        "conditions": [{
                            "targetType": "ORDER_SUBTOTAL",
                            "ids": null,
                            "excludedVariantIds": null,
                            "minimumAmount": 10000.0,
                            "minimumQuantity": null
                        }],
                        "targets": [{
                            "targetType": "ORDER_SUBTOTAL",
                            "excludedVariantIds": null,
                            "id": null,
                            "quantity": null
                        }],
                        "value": {
                            "type": "PERCENTAGE",
                            "value": 10.0,
                            "appliesToEachItem": null
                        }
                    }
                ],
                "discount_application_strategy": "first"
            }
        "#;

        let expected_output: serde_json::Value = serde_json::from_str(expected_json).unwrap();
        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
