use crate::behaviour::ArithmeticGateBehaviour;
use crate::reactive::arithmetic_gate::ArithmeticGateFunction;

pub struct MaxGate {}
impl MaxGate {
}
impl<'a> ArithmeticGateBehaviour<'a> for MaxGate {
    const TYPE_NAME_1: &'static str = "max";
    const OPERATION: ArithmeticGateFunction<i64> = |lhs, rhs| std::cmp::max(lhs,rhs);
}
