use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum SubroutineCallStatement {
    SubroutineCall(Box<(SubroutineCall, Symbol)>),
    Function(Box<SubroutineCallStatementFunction>),
}

#[derive(Clone, Debug, Node)]
pub struct SubroutineCallStatementFunction {
    pub nodes: (Keyword, Symbol, Paren<FunctionSubroutineCall>, Symbol),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn subroutine_call_statement(s: Span) -> IResult<Span, SubroutineCallStatement> {
    alt((
        map(pair(subroutine_call, symbol(";")), |x| {
            SubroutineCallStatement::SubroutineCall(Box::new(x))
        }),
        subroutine_call_statement_function,
    ))(s)
}

#[parser]
pub(crate) fn subroutine_call_statement_function(s: Span) -> IResult<Span, SubroutineCallStatement> {
    let (s, a) = keyword("void")(s)?;
    let (s, b) = symbol("'")(s)?;
    let (s, c) = paren(function_subroutine_call)(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        SubroutineCallStatement::Function(Box::new(SubroutineCallStatementFunction {
            nodes: (a, b, c, d),
        })),
    ))
}

// -----------------------------------------------------------------------------
