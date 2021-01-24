use crate::behaviour::ArithmeticGateBehaviour;
use crate::reactive::arithmetic_gate::ArithmeticGateFunction;

pub struct DivGate {}
impl DivGate {
}
impl<'a> ArithmeticGateBehaviour<'a> for DivGate {
    const TYPE_NAME_1: &'static str = "div";
    const OPERATION: ArithmeticGateFunction<i64> = |lhs, rhs| if rhs != 0 { lhs / rhs } else { 0 };
}
