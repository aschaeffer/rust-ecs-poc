use crate::behaviour::LogicalGateBehaviour;
use crate::reactive::logical_gate::LogicalGateFunction;

pub struct NorGate {}
impl NorGate {
}
impl<'a> LogicalGateBehaviour<'a> for NorGate {
    const TYPE_NAME_1: &'static str = "nor";
    const OPERATION: LogicalGateFunction = |lhs, rhs| !(lhs || rhs);
}
