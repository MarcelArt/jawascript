#[derive(Debug)]
pub enum Expr {
    Number(i64),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}