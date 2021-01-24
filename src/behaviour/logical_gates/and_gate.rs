use crate::behaviour::LogicalGateBehaviour;
use crate::reactive::logical_gate::LogicalGateFunction;

pub struct AndGate {}
impl AndGate {
}
impl<'a> LogicalGateBehaviour<'a> for AndGate {
    const TYPE_NAME_1: &'static str = "and";
    const OPERATION: LogicalGateFunction = |lhs, rhs| lhs && rhs;
}
