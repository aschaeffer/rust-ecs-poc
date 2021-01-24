use crate::behaviour::ArithmeticGateBehaviour;
use crate::reactive::arithmetic_gate::ArithmeticGateFunction;

pub struct ModGate {}
impl ModGate {
}
impl<'a> ArithmeticGateBehaviour<'a> for ModGate {
    const TYPE_NAME_1: &'static str = "mod";
    const OPERATION: ArithmeticGateFunction<i64> = |lhs, rhs| if rhs != 0 { lhs % rhs } else { 0 };
}
