use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct TanhGate {}
impl TanhGate {
}
impl<'a> NumericOperationBehaviour<'a> for TanhGate {
    const TYPE_NAME_1: &'static str = "tanh";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.tanh();
}
