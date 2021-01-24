use crate::behaviour::ArithmeticGateBehaviour;
use crate::reactive::arithmetic_gate::ArithmeticGateFunction;

pub struct MinGate {}
impl MinGate {
}
impl<'a> ArithmeticGateBehaviour<'a> for MinGate {
    const TYPE_NAME_1: &'static str = "min";
    const OPERATION: ArithmeticGateFunction<i64> = |lhs, rhs| std::cmp::min(lhs,rhs);
}
