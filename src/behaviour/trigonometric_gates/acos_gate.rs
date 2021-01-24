use crate::behaviour::NumericOperationBehaviour;
use crate::reactive::NumericOperationFunction;

pub struct AcosGate {}
impl AcosGate {
}
impl<'a> NumericOperationBehaviour<'a> for AcosGate {
    const TYPE_NAME_1: &'static str = "acos";
    const OPERATION: NumericOperationFunction = |lhs: f64| lhs.acos();
}
