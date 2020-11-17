use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    commands: HashMap<String, Rc<dyn ForthClosure>>,
}

trait ForthClosure: Fn(&mut Vec<Value>) -> ForthResult {
    fn clone(&self) -> Rc<dyn ForthClosure>;
}

impl<T> ForthClosure for T
where
    T: Clone + 'static + Fn(&mut Vec<Value>) -> ForthResult,
{
    fn clone(&self) -> Rc<dyn ForthClosure> {
        Rc::new(std::clone::Clone::clone(self))
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

macro_rules! simplecommand {
    ($name:ident, $action:expr) => {
        fn $name(v: &mut Vec<Value>) -> ForthResult {
            if let (Some(ultimate), Some(penultimate)) = (v.pop(), v.pop()) {
                let result = $action(penultimate, ultimate);
                v.push(result);
                return Ok(());
            }
            Err(Error::StackUnderflow)
        }
    };
}

simplecommand!(add, std::ops::Add::add);
simplecommand!(sub, std::ops::Sub::sub);
simplecommand!(mul, std::ops::Mul::mul);

impl Forth {
    pub fn new() -> Forth {
        let mut commands = <HashMap<_, Rc<dyn ForthClosure>>>::new();
        commands.insert("+".into(), Rc::from(add));
        commands.insert("-".into(), Rc::from(sub));
        commands.insert("*".into(), Rc::from(mul));
        commands.insert(
            "/".into(),
            Rc::new(|v: &mut Vec<Value>| match (v.pop(), v.pop()) {
                (Some(0), Some(_)) => Err(Error::DivisionByZero),
                (Some(ultimate), Some(penultimate)) => {
                    let result = penultimate / ultimate;
                    v.push(result);
                    Ok(())
                }
                _ => Err(Error::StackUnderflow),
            }),
        );
        commands.insert(
            "DUP".into(),
            Rc::from(|v: &mut Vec<Value>| {
                if let Some(&last) = v.last() {
                    v.push(last);
                    return Ok(());
                }
                Err(Error::StackUnderflow)
            }),
        );
        commands.insert(
            "DROP".into(),
            Rc::from(|v: &mut Vec<Value>| {
                if v.pop().is_some() {
                    return Ok(());
                }
                Err(Error::StackUnderflow)
            }),
        );
        commands.insert(
            "SWAP".into(),
            Rc::from(|v: &mut Vec<Value>| {
                if let Some(ultimate) = v.pop() {
                    if let Some(penultimate) = v.pop() {
                        v.push(ultimate);
                        v.push(penultimate);
                        return Ok(());
                    }
                };
                Err(Error::StackUnderflow)
            }),
        );
        commands.insert(
            "OVER".into(),
            Rc::new(|v: &mut Vec<Value>| {
                if let Some(ultimate) = v.pop() {
                    if let Some(&penultimate) = v.last() {
                        v.push(ultimate);
                        v.push(penultimate);
                        return Ok(());
                    }
                };
                Err(Error::StackUnderflow)
            }),
        );
        Forth {
            commands,
            ..Default::default()
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &'static str) -> ForthResult {
        let input = input.to_uppercase();
        let input = input.trim();

        let mut iter = input.split_whitespace();
        while let Some(word) = iter.next() {
            match word {
                ":" => {
                    let name = match iter.next() {
                        Some(word) if word.parse::<Value>().is_err() => word,
                        _ => return Err(Error::InvalidWord),
                    };
                    let mut clos: Rc<dyn ForthClosure> = Rc::new(|_v: &mut Vec<Value>| Ok(()));
                    loop {
                        let newcmd = match iter.next() {
                            None => Err(Error::InvalidWord),
                            Some(";") => break,
                            Some(word) if self.commands.get(word).is_some() => {
                                Ok(self.commands[word].clone())
                            }
                            Some(word) if word.parse::<Value>().is_ok() => {
                                let word = word.parse::<Value>().unwrap();
                                let clos: Rc<dyn ForthClosure> =
                                    Rc::new(move |v: &mut Vec<Value>| {
                                        v.push(word);
                                        Ok(())
                                    });
                                Ok(clos)
                            }
                            _ => Err(Error::UnknownWord),
                        }?;
                        clos = Rc::new(move |v: &mut Vec<Value>| clos(v).and_then(|_| newcmd(v)));
                    }
                    self.commands.insert(name.to_string(), clos);
                }
                _ if self.commands.get(word).is_some() => {
                    let cmd = &self.commands[word];
                    cmd(&mut self.stack)?;
                }
                _ if word.parse::<Value>().is_ok() => {
                    self.stack.push(word.parse::<Value>().unwrap())
                }
                _ => return Err(Error::UnknownWord),
            }
        }
        Ok(())
    }
}
