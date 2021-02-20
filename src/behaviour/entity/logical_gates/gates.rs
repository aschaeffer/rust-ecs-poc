use crate::entity_behaviour;

macro_rules! logical_gate {
    ($behaviour_name:ident, $entity_type_name:expr, $f:expr) => {
        entity_behaviour! {
            $behaviour_name,
            crate::behaviour::LogicalGateBehaviour,
            crate::reactive::LogicalGateFunction,
            $entity_type_name,
            $f
        }
    }
}

logical_gate! { AndGate, "and", |lhs, rhs| lhs && rhs }
logical_gate! { NandGate, "nand", |lhs, rhs| !(lhs && rhs) }
logical_gate! { NorGate, "nor", |lhs, rhs| !(lhs || rhs) }
logical_gate! { NotGate, "not", |lhs, _rhs| !lhs }
logical_gate! { OrGate, "or", |lhs, rhs| lhs || rhs }
logical_gate! { XorGate, "xor", |lhs, rhs| lhs ^ rhs }
logical_gate! { XnorGate, "xnor", |lhs, rhs| !(lhs ^ rhs) }
