use crate::errors::CalcErrors;

use super::opcode::Opcode;


pub struct FactoryOp;

impl FactoryOp {
    pub fn match_(op: Opcode, left: f64, right: f64) -> Result<f64, CalcErrors> {
        match op {
            Opcode::Mul => Mul::ahead(left, right),
            Opcode::Div => Div::ahead(left, right),
            Opcode::Mod => Mod::ahead(left, right),
            Opcode::IntDiv => IntDiv::ahead(left, right),
            Opcode::Add => Add::ahead(left, right),
            Opcode::Sub => Sub::ahead(left, right),
        }
    }
}


pub trait Operation {
    fn ahead(left: f64, right: f64) -> Result<f64, CalcErrors>;
}

pub struct Add;

impl Operation for Add {
    fn ahead(left: f64, right: f64) -> Result<f64, CalcErrors> {
        Ok(left + right)
    }
}

pub struct Sub;

impl Operation for Sub {
    fn ahead(left: f64, right: f64) -> Result<f64, CalcErrors> {
        Ok(left - right)
    }
}

pub struct IntDiv;

impl Operation for IntDiv {
    fn ahead(left: f64, right: f64) -> Result<f64, CalcErrors> {
        if right == 0.0 {
            return Err(CalcErrors::DivisionZeroProhibited)
        }
        Ok((left / right).trunc())
    }
}

pub struct Mod;

impl Operation for Mod {
    fn ahead(left: f64, right: f64) -> Result<f64, CalcErrors> {
        Ok(left % right)
    }
}

pub struct Mul;

impl Operation for Mul {
    fn ahead(left: f64, right: f64) -> Result<f64, CalcErrors> {
        Ok(left * right)
    }
}

pub struct Div;

impl Operation for Div {
    fn ahead(left: f64, right: f64) -> Result<f64, CalcErrors> {
        if right == 0.0 {
            return Err(CalcErrors::DivisionZeroProhibited);
        }
        Ok(left / right)
    }
}








