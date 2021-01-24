use crate::behaviour::ArithmeticGateBehaviour;
use crate::reactive::arithmetic_gate::ArithmeticGateFunction;

pub struct MulGate {}
impl MulGate {
}
impl<'a> ArithmeticGateBehaviour<'a> for MulGate {
    const TYPE_NAME_1: &'static str = "mul";
    const OPERATION: ArithmeticGateFunction<i64> = |lhs, rhs| lhs * rhs;
}
