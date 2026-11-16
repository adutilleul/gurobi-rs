//! Most commonly used items from this crate bundled for convenient import.

pub use crate::{
    Constr,
    ConstrSense,
    // ----------
    Env,
    GenConstr,
    INFINITY,
    Model,
    ModelObject,
    ModelSense,
    QConstr,
    RelaxType,
    SOS,
    SOSType,
    Status,
    Var,
    VarSpec,
    // constants
    VarType,
    add_binvar,
    // proc macros
    add_ctsvar,
    add_intvar,
    add_var,
    attr,
    c,
    callback::{Callback, Where},
    constants::Norm,
    expr::{AttachModel, Expr, GurobiSum},
    param,
};

#[cfg(feature = "gurobi13")]
pub use crate::GRB_METHOD_PDHG;

#[cfg(feature = "gurobi14")]
pub use crate::{GRB_METHOD_NLBARRIER, NLConstr, NLExprTree};

pub use ModelSense::*;
pub use VarType::*;
