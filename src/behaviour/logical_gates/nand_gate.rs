use crate::behaviour::LogicalGateBehaviour;
use crate::reactive::logical_gate::LogicalGateFunction;

pub struct NandGate {}
impl NandGate {
}
impl<'a> LogicalGateBehaviour<'a> for NandGate {
    const TYPE_NAME_1: &'static str = "nand";
    const OPERATION: LogicalGateFunction = |lhs, rhs| !(lhs && rhs);
}
