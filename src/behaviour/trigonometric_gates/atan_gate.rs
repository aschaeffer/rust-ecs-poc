use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct AtanGate {}
impl AtanGate {
}
impl<'a> NumericOperationBehaviour<'a> for AtanGate {
    const TYPE_NAME_1: &'static str = "atan";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.atan();
}
