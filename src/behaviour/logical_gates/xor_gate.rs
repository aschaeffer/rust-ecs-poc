use crate::behaviour::LogicalGateBehaviour;
use crate::reactive::logical_gate::LogicalGateFunction;

pub struct XorGate {}
impl XorGate {
}
impl<'a> LogicalGateBehaviour<'a> for XorGate {
    const TYPE_NAME_1: &'static str = "xor";
    const OPERATION: LogicalGateFunction = |lhs, rhs| lhs ^ rhs;
}
