use crate::behaviour::ArithmeticGateBehaviour;
use crate::reactive::arithmetic_gate::ArithmeticGateFunction;

pub struct SubGate {}
impl SubGate {
}
impl<'a> ArithmeticGateBehaviour<'a> for SubGate {
    const TYPE_NAME_1: &'static str = "sub";
    const OPERATION: ArithmeticGateFunction<i64> = |lhs, rhs| lhs - rhs;
}
