use crate::entity_behaviour;

macro_rules! arithmetic_gate {
    ($behaviour_name:ident, $entity_type_name:expr, $f:expr) => {
        entity_behaviour! {
            $behaviour_name,
            crate::behaviour::ArithmeticGateBehaviour,
            crate::reactive::entity::arithmetic_gate::ArithmeticGateFunction<i64>,
            $entity_type_name,
            $f
        }
    }
}

// TODO: all of them: i64, f64
arithmetic_gate! { AddGate, "add", |lhs, rhs| lhs + rhs }
arithmetic_gate! { DivGate, "div", |lhs, rhs| if rhs != 0 { lhs / rhs } else { 0 } }
arithmetic_gate! { MaxGate, "max", |lhs, rhs| std::cmp::max(lhs,rhs) }
arithmetic_gate! { MinGate, "min", |lhs, rhs| std::cmp::min(lhs,rhs) }
arithmetic_gate! { ModGate, "mod", |lhs, rhs| if rhs != 0 { lhs % rhs } else { 0 } }
arithmetic_gate! { MulGate, "mul", |lhs, rhs| lhs * rhs }
arithmetic_gate! { SubGate, "sub", |lhs, rhs| lhs - rhs }
