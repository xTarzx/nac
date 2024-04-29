use anyhow::{anyhow, Result};

#[derive(Debug, Default)]

pub struct OpExpression {
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Expression {
    Unit(String),
    Add(OpExpression),
    Sub(OpExpression),
    Mul(OpExpression),
    Div(OpExpression),
    Mod(OpExpression),
    Pow(OpExpression),
    Group(Group),
}

impl Expression {
    pub fn root(input: &str) -> Result<Expression> {
        let body = tokenize(input)?;
        Ok(Expression::Group(Group { body }))
    }

    pub fn eval(&mut self) -> Result<f64> {
        match self {
            Expression::Unit(val) => {
                let value: f64 = val.parse()?;
                return Ok(value);
            }
            Expression::Add(OpExpression {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs + rhs);
            }
            Expression::Sub(OpExpression {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs - rhs);
            }
            Expression::Mul(OpExpression {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs * rhs);
            }
            Expression::Div(OpExpression {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs / rhs);
            }
            Expression::Mod(OpExpression {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs % rhs);
            }

            Expression::Pow(OpExpression {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs.powf(rhs));
            }

            Expression::Group(group) => {
                group.resolve()?;

                if group.body.len() != 1 {
                    return Err(anyhow!("unresolved expression {group:?}"));
                }

                return group.body[0].eval();
            }
            _ => {
                return Err(anyhow!("unhandled expression {self:?}"));
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Group {
    body: Vec<Expression>,
}

impl Group {
    fn resolve(&mut self) -> Result<()> {
        while let Some(idx) = self.body.iter().position(|e| match e {
            Expression::Pow(params) => params.lhs.is_none() || params.lhs.is_none(),
            _ => false,
        }) {
            if idx + 1 == self.body.len() {
                return Err(anyhow!("missing right hand side"));
            }
            let rhs = self.body.remove(idx + 1);
            if idx == 0 {
                return Err(anyhow!("missing left hand side"));
            }
            let lhs = self.body.remove(idx - 1);

            let exp = &mut self.body[idx - 1];

            let params = match exp {
                Expression::Pow(params) => params,
                _ => unreachable!(),
            };

            params.lhs = Some(Box::new(lhs));
            params.rhs = Some(Box::new(rhs));
        }

        while let Some(idx) = self.body.iter().position(|e| match e {
            Expression::Mul(params) => params.lhs.is_none() || params.lhs.is_none(),
            Expression::Div(params) => params.lhs.is_none() || params.lhs.is_none(),
            Expression::Mod(params) => params.lhs.is_none() || params.lhs.is_none(),
            _ => false,
        }) {
            if idx + 1 == self.body.len() {
                return Err(anyhow!("missing right hand side"));
            }
            let rhs = self.body.remove(idx + 1);
            if idx == 0 {
                return Err(anyhow!("missing left hand side"));
            }
            let lhs = self.body.remove(idx - 1);

            let exp = &mut self.body[idx - 1];

            let params = match exp {
                Expression::Mul(params) => params,
                Expression::Div(params) => params,
                Expression::Mod(params) => params,
                _ => unreachable!(),
            };

            params.lhs = Some(Box::new(lhs));
            params.rhs = Some(Box::new(rhs));
        }

        while let Some(idx) = self.body.iter().position(|e| match e {
            Expression::Add(params) => params.lhs.is_none() || params.lhs.is_none(),
            Expression::Sub(params) => params.lhs.is_none() || params.lhs.is_none(),
            _ => false,
        }) {
            if idx + 1 == self.body.len() {
                return Err(anyhow!("missing right hand side"));
            }
            let rhs = self.body.remove(idx + 1);
            if idx == 0 {
                return Err(anyhow!("missing left hand side"));
            }
            let lhs = self.body.remove(idx - 1);

            let exp = &mut self.body[idx - 1];

            let params = match exp {
                Expression::Add(params) => params,
                Expression::Sub(params) => params,
                _ => unreachable!(),
            };

            params.lhs = Some(Box::new(lhs));
            params.rhs = Some(Box::new(rhs));
        }
        Ok(())
    }
}

fn tokenize(input: &str) -> Result<Vec<Expression>> {
    let mut exps = vec![];

    let mut chars = input.chars().into_iter().peekable();

    while let Some(char) = chars.next() {
        if char.is_whitespace() {
            continue;
        };

        if char.is_digit(10) {
            let mut buf = String::new();
            buf.push(char);

            while let Some(nxt) = chars.peek() {
                if nxt.is_digit(10) {
                    buf.push(chars.next().unwrap())
                } else {
                    break;
                }
            }

            exps.push(Expression::Unit(buf));
        } else if char == '+' {
            let ops = OpExpression::default();
            exps.push(Expression::Add(ops));
        } else if char == '-' {
            let ops = OpExpression::default();
            exps.push(Expression::Sub(ops));
        } else if char == '*' {
            let ops = OpExpression::default();
            exps.push(Expression::Mul(ops));
        } else if char == '/' {
            let ops = OpExpression::default();
            exps.push(Expression::Div(ops));
        } else if char == '%' {
            let ops = OpExpression::default();
            exps.push(Expression::Mod(ops));
        } else if char == '^' {
            let ops = OpExpression::default();
            exps.push(Expression::Pow(ops));
        } else if char == '(' {
            let mut sc = 0;
            let mut buf = String::new();

            'parse_paren: loop {
                let c = chars.next();
                if c.is_none() {
                    return Err(anyhow!("someone forgot a )"));
                }

                let c = c.unwrap();

                if c == ')' {
                    if sc == 0 {
                        break 'parse_paren;
                    } else {
                        sc -= 1;
                    }
                }

                if c == '(' {
                    sc += 1
                };

                buf.push(c);
            }

            let body = tokenize(&buf)?;

            exps.push(Expression::Group(Group { body }));
        } else if char == ')' {
            return Err(anyhow!("sneaky {char}"));
        } else {
            return Err(anyhow!("unexpected character {char}"));
        }
    }

    return Ok(exps);
}
