use crate::entity_behaviour;

macro_rules! numeric_operation {
    ($behaviour_name:ident, $entity_type_name:expr, $f:expr) => {
        entity_behaviour! {
            $behaviour_name,
            crate::behaviour::NumericOperationBehaviour,
            crate::reactive::NumericOperationFunction<f64>,
            $entity_type_name,
            $f
        }
    }
}

numeric_operation! { AcosGate, "acos", |lhs: f64| lhs.acos() }
numeric_operation! { AsinGate, "asin", |lhs: f64| lhs.asin() }
numeric_operation! { AtanGate, "atan", |lhs: f64| lhs.atan() }
numeric_operation! { CosGate, "cos", |lhs: f64| lhs.cos() }
numeric_operation! { CoshGate, "cosh", |lhs: f64| lhs.cosh() }
numeric_operation! { SinGate, "sin", |lhs: f64| lhs.sin() }
numeric_operation! { SinhGate, "sinh", |lhs: f64| lhs.sinh() }
numeric_operation! { TanGate, "tan", |lhs: f64| lhs.tan() }
numeric_operation! { TanhGate, "tanh", |lhs: f64| lhs.tanh() }

numeric_operation! { FloorGate, "floor", |lhs: f64| lhs.ceil() }
numeric_operation! { CeilGate, "ceil", |lhs: f64| lhs.ceil() }
numeric_operation! { RoundGate, "round", |lhs: f64| lhs.round() }
numeric_operation! { TruncGate, "trunc", |lhs: f64| lhs.trunc() }
numeric_operation! { AbsGate, "abs", |lhs: f64| lhs.abs() }
numeric_operation! { SignumGate, "signum", |lhs: f64| lhs.signum() }
numeric_operation! { SqrtGate, "sqrt", |lhs: f64| lhs.sqrt() }
numeric_operation! { ExpGate, "exp", |lhs: f64| lhs.exp() }
numeric_operation! { Exp2Gate, "exp2", |lhs: f64| lhs.exp2() }
numeric_operation! { LnGate, "ln", |lhs: f64| lhs.ln() }
numeric_operation! { Log2Gate, "log2", |lhs: f64| lhs.log2() }
numeric_operation! { Log10Gate, "log10", |lhs: f64| lhs.log10() }
numeric_operation! { CbrtGate, "cbrt", |lhs: f64| lhs.cbrt() }
numeric_operation! { RecipGate, "recip", |lhs: f64| lhs.recip() }
numeric_operation! { ToDegreesGate, "to_degrees", |lhs: f64| lhs.to_degrees() }
numeric_operation! { ToRadiansGate, "to_radians", |lhs: f64| lhs.to_radians() }





// is_sign_negative -> input f64, output bool
