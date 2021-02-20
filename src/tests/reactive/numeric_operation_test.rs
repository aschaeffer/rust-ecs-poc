use std::f64::consts::PI;

use serde_json::json;

use crate::api::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::behaviour::{CosGate, EntityBehaviour, NumericOperationBehaviour, SinGate};
use crate::model::ReactiveEntityInstanceFactory;
use crate::reactive::entity::numeric_operation::{NumericOperation, NumericOperationProperties};
use crate::reactive::entity::numeric_operation::NumericOperationReactiveEntityInstanceFactory as Factory;

const LHS: NumericOperationProperties = NumericOperationProperties::LHS;
const RESULT: NumericOperationProperties = NumericOperationProperties::RESULT;

#[test]
fn numeric_operation_sin_type_test () {
    let sin_entity = Factory::new(SinGate::TYPE_NAME);
    let sin_gate = NumericOperation::new(sin_entity.clone(), SinGate::OPERATION);
    assert_eq!(SinGate::TYPE_NAME.to_string(), sin_gate.type_name());
}

#[test]
fn numeric_operation_sin_test () {
    let sin_entity = Factory::new(SinGate::TYPE_NAME);
    {
        let sin_gate = NumericOperation::new(sin_entity.clone(), SinGate::OPERATION);
        assert_ne!(0, sin_gate.handle_id);

        sin_entity.set(LHS.to_string(), json!(0.0));
        assert_eq!(0.0, sin_entity.as_f64(RESULT.to_string()).unwrap());

        sin_entity.set(LHS.to_string(), json!(PI / 2.0));
        assert_eq!(1.0, sin_entity.as_f64(RESULT.to_string()).unwrap());

    } // The TrigonometricGate no more alive ...
    sin_entity.set(LHS.to_string(), json!(0.0));
    assert_eq!(1.0, sin_entity.as_f64(RESULT.to_string()).unwrap());
}

#[test]
fn numeric_operation_cos_type_test () {
    let cos_entity = Factory::new(CosGate::TYPE_NAME);
    let cos_gate = NumericOperation::new(cos_entity.clone(), CosGate::OPERATION);
    assert_eq!(CosGate::TYPE_NAME.to_string(), cos_gate.type_name());
}

#[test]
fn numeric_operation_cos_test () {
    let cos_entity = Factory::new(CosGate::TYPE_NAME);
    {
        let cos_gate = NumericOperation::new(cos_entity.clone(), CosGate::OPERATION);
        assert_ne!(0, cos_gate.handle_id);

        cos_entity.set(LHS.to_string(), json!(0.0));
        assert_eq!(1.0, cos_entity.as_f64(RESULT.to_string()).unwrap());

        cos_entity.set(LHS.to_string(), json!(PI / 2.0));
        assert!(assert_approx(0.0, cos_entity.as_f64(RESULT.to_string()).unwrap()));
    } // The TrigonometricGate no more alive ...
    cos_entity.set(LHS.to_string(), json!(0.0));
    assert!(assert_approx(0.0, cos_entity.as_f64(RESULT.to_string()).unwrap()));
}

fn assert_approx(expected: f64, value: f64) -> bool {
    value > expected - 0.00000001 && value < expected + 0.00000001
}
