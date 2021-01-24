use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct TanGate {}
impl TanGate {
}
impl<'a> NumericOperationBehaviour<'a> for TanGate {
    const TYPE_NAME_1: &'static str = "tan";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.tan();
}
