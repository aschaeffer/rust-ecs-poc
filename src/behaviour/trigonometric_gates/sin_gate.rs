use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct SinGate {}
impl SinGate {
}
impl<'a> NumericOperationBehaviour<'a> for SinGate {
    const TYPE_NAME_1: &'static str = "sin";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.sin();
}
