use crate::behaviour::ArithmeticGateBehaviour;
use crate::reactive::arithmetic_gate::ArithmeticGateFunction;

pub struct AddGate {}
impl AddGate {
}
impl<'a> ArithmeticGateBehaviour<'a> for AddGate {
    const TYPE_NAME_1: &'static str = "add";
    const OPERATION: ArithmeticGateFunction<i64> = |lhs, rhs| lhs + rhs;
}
