use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct AsinGate {}
impl AsinGate {
}
impl<'a> NumericOperationBehaviour<'a> for AsinGate {
    const TYPE_NAME_1: &'static str = "asin";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.asin();
}
