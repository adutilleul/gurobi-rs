#![cfg(feature = "gurobi14")]

use gurobi_rs::prelude::*;
use gurobi_rs::{ConstrSense, Opcode};

#[test]
fn nonlinear_constraint_and_objective_roundtrip() -> anyhow::Result<()> {
    let mut model = Model::new("nlconstr")?;
    let x = add_ctsvar!(model, name: "x", bounds: -10.0..10.0)?;
    let y = add_ctsvar!(model, name: "y")?;
    model.update()?;

    let x_idx = model.var_index(&x)? as f64;
    let y_idx = model.var_index(&y)? as f64;

    // y - x^2 >= 0
    let opcode = vec![
        Opcode::Minus as i32,
        Opcode::Variable as i32,
        Opcode::Square as i32,
        Opcode::Variable as i32,
    ];
    let data = vec![0.0, y_idx, 0.0, x_idx];
    let parent = vec![-1, 0, 0, 2];

    let constr = model.add_nlconstr("nl", &opcode, &data, &parent, ConstrSense::Greater, 0.0)?;
    model.update()?;

    assert_eq!(model.get_attr(attr::NumNLConstrs)?, 1);
    assert_eq!(model.get_attr(attr::IsNLP)?, 1);
    assert_eq!(model.get_obj_attr(attr::NLCName, &constr)?, "nl");
    assert_eq!(
        model.get_obj_attr(attr::NLCSense, &constr)?,
        ConstrSense::Greater
    );
    assert_eq!(model.get_obj_attr(attr::NLCRHS, &constr)?, 0.0);
    assert_eq!(model.get_nlconstr_by_name("nl")?, Some(constr));
    assert_eq!(model.get_nlconstr_by_name("missing")?, None);

    let tree = model.get_nlconstr(&constr)?;
    assert_eq!(tree.opcode.len(), opcode.len());
    assert_eq!(tree.data.len(), data.len());
    assert_eq!(tree.parent.len(), parent.len());
    assert_eq!(tree.parent[0], -1);

    // Set and clear a nonlinear objective.
    model.set_nlobj(&opcode, &data, &parent)?;
    model.update()?;
    assert!(model.get_nlobj()?.is_some());
    model.del_nlobj()?;
    model.update()?;
    assert_eq!(model.get_nlobj()?, None);

    model.remove(constr)?;
    model.update()?;
    assert_eq!(model.get_attr(attr::NumNLConstrs)?, 0);

    Ok(())
}
