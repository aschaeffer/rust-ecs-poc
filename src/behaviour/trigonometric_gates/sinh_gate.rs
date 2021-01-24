use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct SinhGate {}
impl SinhGate {
}
impl<'a> NumericOperationBehaviour<'a> for SinhGate {
    const TYPE_NAME_1: &'static str = "sinh";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.sinh();
}
