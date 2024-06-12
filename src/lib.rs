use anyhow::{anyhow, Result};

#[derive(Debug, Default)]

pub struct OpParams {
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Expression {
    Unit(String),
    Add(OpParams),
    Sub(OpParams),
    Mul(OpParams),
    Div(OpParams),
    Mod(OpParams),
    Pow(OpParams),
    Root(OpParams),
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
            Expression::Add(OpParams {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs + rhs);
            }
            Expression::Sub(OpParams {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs - rhs);
            }
            Expression::Mul(OpParams {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs * rhs);
            }
            Expression::Div(OpParams {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs / rhs);
            }
            Expression::Mod(OpParams {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs % rhs);
            }

            Expression::Pow(OpParams {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(lhs.powf(rhs));
            }
            Expression::Root(OpParams {
                lhs: Some(lhs),
                rhs: Some(rhs),
            }) => {
                let lhs = lhs.eval()?;
                let rhs = rhs.eval()?;
                return Ok(rhs.powf(1.0 / lhs));
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
    fn parse_params(&mut self, mut exp_idx: usize) -> Result<()> {
        if exp_idx + 1 == self.body.len() {
            return Err(anyhow!("missing right hand side"));
        }

        let rhs = self.body.remove(exp_idx + 1);

        let lhs: Option<Expression>;
        if exp_idx == 0 {
            lhs = None;
        } else {
            exp_idx -= 1;
            lhs = Some(self.body.remove(exp_idx));
        };

        let exp = &mut self.body[exp_idx];

        match exp {
            Expression::Add(params) | Expression::Sub(params) => {
                params.lhs = Some(Box::new(lhs.unwrap_or(Expression::Unit("0".to_string()))));
                params.rhs = Some(Box::new(rhs));
            }
            Expression::Mul(params)
            | Expression::Div(params)
            | Expression::Mod(params)
            | Expression::Pow(params)
            | Expression::Root(params) => {
                if lhs.is_none() {
                    return Err(anyhow!("missing left hand side"));
                }

                let lhs = lhs.unwrap();
                params.lhs = Some(Box::new(lhs));
                params.rhs = Some(Box::new(rhs));
            }
            _ => return Err(anyhow!("unexpected expression {exp:?}")),
        }

        Ok(())
    }

    fn resolve(&mut self) -> Result<()> {
        while let Some(idx) = self.body.iter().position(|e| match e {
            Expression::Pow(params) => params.lhs.is_none() || params.lhs.is_none(),
            Expression::Root(params) => params.lhs.is_none() || params.lhs.is_none(),
            _ => false,
        }) {
            self.parse_params(idx)?;
        }

        while let Some(idx) = self.body.iter().position(|e| match e {
            Expression::Mul(params) => params.lhs.is_none() || params.lhs.is_none(),
            Expression::Div(params) => params.lhs.is_none() || params.lhs.is_none(),
            Expression::Mod(params) => params.lhs.is_none() || params.lhs.is_none(),
            _ => false,
        }) {
            self.parse_params(idx)?;
        }

        while let Some(idx) = self.body.iter().position(|e| match e {
            Expression::Add(params) => params.lhs.is_none() || params.lhs.is_none(),
            Expression::Sub(params) => params.lhs.is_none() || params.lhs.is_none(),
            _ => false,
        }) {
            self.parse_params(idx)?;
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
                if nxt.is_digit(10) || nxt == &'.' {
                    buf.push(chars.next().unwrap())
                } else {
                    break;
                }
            }

            if buf.ends_with('.') {
                buf.push('0');
            }

            exps.push(Expression::Unit(buf));
        } else if char == '#' {
            let mut buf = String::new();

            while let Some(nxt) = chars.peek() {
                if nxt.is_digit(16) {
                    buf.push(chars.next().unwrap())
                } else {
                    break;
                }
            }

            let val = u64::from_str_radix(buf.as_str(), 16)?;
            exps.push(Expression::Unit(val.to_string()));
        } else if char == '+' {
            let ops = OpParams::default();
            exps.push(Expression::Add(ops));
        } else if char == '-' {
            let ops = OpParams::default();
            exps.push(Expression::Sub(ops));
        } else if char == '*' {
            let ops = OpParams::default();
            exps.push(Expression::Mul(ops));
        } else if char == '/' {
            let ops = OpParams::default();
            exps.push(Expression::Div(ops));
        } else if char == '%' {
            let ops = OpParams::default();
            exps.push(Expression::Mod(ops));
        } else if char == '^' {
            let ops = OpParams::default();
            exps.push(Expression::Pow(ops));
        } else if char == '~' {
            let ops = OpParams::default();
            exps.push(Expression::Root(ops));
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
