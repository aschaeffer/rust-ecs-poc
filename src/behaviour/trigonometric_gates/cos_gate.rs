use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct CosGate {}
impl CosGate {
}
impl<'a> NumericOperationBehaviour<'a> for CosGate {
    const TYPE_NAME_1: &'static str = "cos";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.cos();
}
