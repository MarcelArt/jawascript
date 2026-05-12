#[derive(Debug)]
pub enum Expr {
    Number(i64),

    Variable(String),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    }
}

#[derive(Debug)]
pub enum Statement {
    Let {
        name: String,
        value: Expr,
    },

    Expr(Expr),

    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    Less,
}