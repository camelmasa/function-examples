pub struct Output;
pub mod output {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Output";
    pub const QUERY: &str =
        "mutation Output($result: HandleResult!) {\n  handleResult(result: $result)\n}\n";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type Void = super::Void;
    #[derive()]
    pub enum DiscountApplicationStrategy {
        FIRST,
        MAXIMUM,
        Other(String),
    }
    impl ::serde::Serialize for DiscountApplicationStrategy {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                DiscountApplicationStrategy::FIRST => "FIRST",
                DiscountApplicationStrategy::MAXIMUM => "MAXIMUM",
                DiscountApplicationStrategy::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DiscountApplicationStrategy {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "FIRST" => Ok(DiscountApplicationStrategy::FIRST),
                "MAXIMUM" => Ok(DiscountApplicationStrategy::MAXIMUM),
                _ => Ok(DiscountApplicationStrategy::Other(s)),
            }
        }
    }
    #[derive()]
    pub enum TargetType {
        ORDER_SUBTOTAL,
        PRODUCT_VARIANT,
        Other(String),
    }
    impl ::serde::Serialize for TargetType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                TargetType::ORDER_SUBTOTAL => "ORDER_SUBTOTAL",
                TargetType::PRODUCT_VARIANT => "PRODUCT_VARIANT",
                TargetType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TargetType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s = <String>::deserialize(deserializer)?;
            match s.as_str() {
                "ORDER_SUBTOTAL" => Ok(TargetType::ORDER_SUBTOTAL),
                "PRODUCT_VARIANT" => Ok(TargetType::PRODUCT_VARIANT),
                _ => Ok(TargetType::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Condition {
        pub productMinimumQuantity: Option<ProductMinimumQuantity>,
        pub productMinimumSubtotal: Option<ProductMinimumSubtotal>,
    }
    #[derive(Serialize)]
    pub struct Discount {
        pub conditions: Option<Vec<Condition>>,
        pub message: Option<String>,
        pub targets: Vec<Target>,
        pub value: Value,
    }
    #[derive(Serialize)]
    pub struct FixedAmount {
        pub appliesToEachItem: Option<Boolean>,
        pub value: Float,
    }
    #[derive(Serialize)]
    pub struct Percentage {
        pub value: Float,
    }
    #[derive(Serialize)]
    pub struct ProductMinimumQuantity {
        pub ids: Vec<ID>,
        pub minimumQuantity: Int,
        pub targetType: TargetType,
    }
    #[derive(Serialize)]
    pub struct ProductMinimumSubtotal {
        pub ids: Vec<ID>,
        pub minimumAmount: Float,
        pub targetType: TargetType,
    }
    #[derive(Serialize)]
    pub struct ProductVariantTarget {
        pub id: ID,
        pub quantity: Option<Int>,
    }
    #[derive(Serialize)]
    pub struct HandleResult {
        pub discountApplicationStrategy: DiscountApplicationStrategy,
        pub discounts: Vec<Discount>,
    }
    #[derive(Serialize)]
    pub struct Target {
        pub productVariant: Option<ProductVariantTarget>,
    }
    #[derive(Serialize)]
    pub struct Value {
        pub fixedAmount: Option<FixedAmount>,
        pub percentage: Option<Percentage>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub result: HandleResult,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "handleResult")]
        pub handle_result: Void,
    }
}
impl graphql_client::GraphQLQuery for Output {
    type Variables = output::Variables;
    type ResponseData = output::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: output::QUERY,
            operation_name: output::OPERATION_NAME,
        }
    }
}
