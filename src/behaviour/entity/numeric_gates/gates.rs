use crate::entity_behaviour;

macro_rules! numeric_gate {
    ($behaviour_name:ident, $entity_type_name:expr, $f:expr) => {
        entity_behaviour! {
            $behaviour_name,
            crate::behaviour::NumericGateBehaviour,
            crate::reactive::entity::numeric_gate::NumericGateFunction<f64>,
            $entity_type_name,
            $f
        }
    }
}

numeric_gate! { PowGate, "pow", |lhs, rhs| lhs.powf(rhs) }

numeric_gate! { LogGate, "log", |lhs, rhs| lhs.log(rhs) }
numeric_gate! { HypotGate, "hypot", |lhs, rhs| lhs.hypot(rhs) }
numeric_gate! { Atan2Gate, "atan2", |lhs, rhs| lhs.atan2(rhs) }

// clamp -> gate with 3 inputs = value, min, max
