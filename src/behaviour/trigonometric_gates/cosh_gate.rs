use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct CoshGate {}
impl CoshGate {
}
impl<'a> NumericOperationBehaviour<'a> for CoshGate {
    const TYPE_NAME_1: &'static str = "cosh";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.cosh();
}
