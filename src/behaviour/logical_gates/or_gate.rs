use crate::behaviour::LogicalGateBehaviour;
use crate::reactive::logical_gate::LogicalGateFunction;

pub struct OrGate {}
impl OrGate {
}
impl<'a> LogicalGateBehaviour<'a> for OrGate {
    const TYPE_NAME_1: &'static str = "or";
    const OPERATION: LogicalGateFunction = |lhs, rhs| lhs || rhs;
}
