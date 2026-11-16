//! Gurobi 14 nonlinear-constraint example.
//!
//! Solves:
//!   minimize y
//!   subject to y >= exp(x),   x in [-2, 2],   y free
//!
//! The left-hand side `y - exp(x)` is encoded as a nonlinear expression tree
//! and added with [`Model::add_nlconstr`].
//!
//! Tree (parent index in brackets):
//!
//!   node 0  MINUS        parent = -1   <-- root, the constraint left-hand side
//!   node 1  VARIABLE y   parent = 0
//!   node 2  EXP          parent = 0
//!   node 3  VARIABLE x   parent = 2

use gurobi_rs::prelude::*;
use gurobi_rs::{ConstrSense, Opcode};

fn main() -> gurobi_rs::Result<()> {
    let mut model = Model::new("nlconstr")?;

    let x = add_ctsvar!(model, name: "x", bounds: -2.0..2.0)?;
    let y = add_ctsvar!(model, name: "y", bounds: -INFINITY..INFINITY)?;
    model.update()?;

    model.set_objective(1.0 * y, Minimize)?;

    let x_idx = model.var_index(&x)? as f64;
    let y_idx = model.var_index(&y)? as f64;

    let opcode = vec![
        Opcode::Minus as i32,
        Opcode::Variable as i32,
        Opcode::Exp as i32,
        Opcode::Variable as i32,
    ];
    let data = vec![0.0, y_idx, 0.0, x_idx];
    let parent = vec![-1, 0, 0, 2];

    let constr =
        model.add_nlconstr("nl_rhs", &opcode, &data, &parent, ConstrSense::Greater, 0.0)?;

    model.optimize()?;

    println!("status = {:?}", model.status()?);
    let x_val = model.get_obj_attr(attr::X, &x)?;
    let y_val = model.get_obj_attr(attr::X, &y)?;
    let obj = model.get_attr(attr::ObjVal)?;
    println!("x = {x_val:.6}");
    println!("y = {y_val:.6}");
    println!("obj = {obj}");
    println!(
        "NLCName = {}, NLCRHS = {}",
        model.get_obj_attr(attr::NLCName, &constr)?,
        model.get_obj_attr(attr::NLCRHS, &constr)?
    );

    Ok(())
}
