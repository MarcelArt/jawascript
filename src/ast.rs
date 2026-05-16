#[derive(Debug)]
pub enum Expr {
    Number(i64),

    Variable(String),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    Call {
        name: String,
        args: Vec<Expr>,
    },
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

    Assign {
        name: String,
        value: Expr,
    },

    While {
        condition: Expr,
        body: Vec<Statement>,
    },

    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
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