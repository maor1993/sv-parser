use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub enum CaseStatement {
    Normal(Box<CaseStatementNormal>),
    Matches(Box<CaseStatementMatches>),
    Inside(Box<CaseStatementInside>),
}

#[derive(Clone, Debug, Node)]
pub struct CaseStatementNormal {
    pub nodes: (
        Option<UniquePriority>,
        CaseKeyword,
        Paren<CaseExpression>,
        CaseItem,
        Vec<CaseItem>,
        Keyword,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct CaseStatementMatches {
    pub nodes: (
        Option<UniquePriority>,
        CaseKeyword,
        Paren<CaseExpression>,
        Keyword,
        CasePatternItem,
        Vec<CasePatternItem>,
        Keyword,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct CaseStatementInside {
    pub nodes: (
        Option<UniquePriority>,
        Keyword,
        Paren<CaseExpression>,
        Keyword,
        CaseInsideItem,
        Vec<CaseInsideItem>,
        Keyword,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum CaseKeyword {
    Case(Box<Keyword>),
    Casez(Box<Keyword>),
    Casex(Box<Keyword>),
}

#[derive(Clone, Debug, Node)]
pub struct CaseExpression {
    pub nodes: (Expression,),
}

#[derive(Clone, Debug, Node)]
pub enum CaseItem {
    NonDefault(Box<CaseItemNondefault>),
    Default(Box<CaseItemDefault>),
}

#[derive(Clone, Debug, Node)]
pub struct CaseItemNondefault {
    pub nodes: (List<Symbol, CaseItemExpression>, Symbol, StatementOrNull),
}

#[derive(Clone, Debug, Node)]
pub struct CaseItemDefault {
    pub nodes: (Keyword, Option<Symbol>, StatementOrNull),
}

#[derive(Clone, Debug, Node)]
pub enum CasePatternItem {
    NonDefault(Box<CasePatternItemNondefault>),
    Default(Box<CaseItemDefault>),
}

#[derive(Clone, Debug, Node)]
pub struct CasePatternItemNondefault {
    pub nodes: (
        Pattern,
        Option<(Symbol, Expression)>,
        Symbol,
        StatementOrNull,
    ),
}

#[derive(Clone, Debug, Node)]
pub enum CaseInsideItem {
    NonDefault(Box<CaseInsideItemNondefault>),
    Default(Box<CaseItemDefault>),
}

#[derive(Clone, Debug, Node)]
pub struct CaseInsideItemNondefault {
    pub nodes: (OpenRangeList, Symbol, StatementOrNull),
}

#[derive(Clone, Debug, Node)]
pub struct CaseItemExpression {
    pub nodes: (Expression,),
}

#[derive(Clone, Debug, Node)]
pub struct RandcaseStatement {
    pub nodes: (Keyword, RandcaseItem, Vec<RandcaseItem>, Keyword),
}

#[derive(Clone, Debug, Node)]
pub struct RandcaseItem {
    pub nodes: (Expression, Symbol, StatementOrNull),
}

#[derive(Clone, Debug, Node)]
pub struct OpenRangeList {
    pub nodes: (List<Symbol, OpenValueRange>,),
}

#[derive(Clone, Debug, Node)]
pub struct OpenValueRange {
    pub nodes: (ValueRange,),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn case_statement(s: Span) -> IResult<Span, CaseStatement> {
    alt((
        case_statement_normal,
        case_statement_matches,
        case_statement_inside,
    ))(s)
}

#[parser]
pub(crate) fn case_statement_normal(s: Span) -> IResult<Span, CaseStatement> {
    let (s, a) = opt(unique_priority)(s)?;
    let (s, b) = case_keyword(s)?;
    let (s, c) = paren(case_expression)(s)?;
    let (s, d) = case_item(s)?;
    let (s, e) = many0(case_item)(s)?;
    let (s, f) = keyword("endcase")(s)?;
    Ok((
        s,
        CaseStatement::Normal(Box::new(CaseStatementNormal {
            nodes: (a, b, c, d, e, f),
        })),
    ))
}

#[parser]
pub(crate) fn case_statement_matches(s: Span) -> IResult<Span, CaseStatement> {
    let (s, a) = opt(unique_priority)(s)?;
    let (s, b) = case_keyword(s)?;
    let (s, c) = paren(case_expression)(s)?;
    let (s, d) = keyword("matches")(s)?;
    let (s, e) = case_pattern_item(s)?;
    let (s, f) = many0(case_pattern_item)(s)?;
    let (s, g) = keyword("endcase")(s)?;
    Ok((
        s,
        CaseStatement::Matches(Box::new(CaseStatementMatches {
            nodes: (a, b, c, d, e, f, g),
        })),
    ))
}

#[parser]
pub(crate) fn case_statement_inside(s: Span) -> IResult<Span, CaseStatement> {
    let (s, a) = opt(unique_priority)(s)?;
    let (s, b) = keyword("case")(s)?;
    let (s, c) = paren(case_expression)(s)?;
    let (s, d) = keyword("inside")(s)?;
    let (s, e) = case_inside_item(s)?;
    let (s, f) = many0(case_inside_item)(s)?;
    let (s, g) = keyword("endcase")(s)?;
    Ok((
        s,
        CaseStatement::Inside(Box::new(CaseStatementInside {
            nodes: (a, b, c, d, e, f, g),
        })),
    ))
}

#[parser]
pub(crate) fn case_keyword(s: Span) -> IResult<Span, CaseKeyword> {
    alt((
        map(keyword("casez"), |x| CaseKeyword::Casez(Box::new(x))),
        map(keyword("casex"), |x| CaseKeyword::Casex(Box::new(x))),
        map(keyword("case"), |x| CaseKeyword::Case(Box::new(x))),
    ))(s)
}

#[parser]
pub(crate) fn case_expression(s: Span) -> IResult<Span, CaseExpression> {
    let (s, a) = expression(s)?;
    Ok((s, CaseExpression { nodes: (a,) }))
}

#[parser]
pub(crate) fn case_item(s: Span) -> IResult<Span, CaseItem> {
    alt((
        case_item_nondefault,
        map(case_item_default, |x| CaseItem::Default(Box::new(x))),
    ))(s)
}

#[parser(MaybeRecursive)]
pub(crate) fn case_item_nondefault(s: Span) -> IResult<Span, CaseItem> {
    let (s, a) = list(symbol(","), case_item_expression)(s)?;
    let (s, b) = symbol(":")(s)?;
    let (s, c) = statement_or_null(s)?;
    Ok((
        s,
        CaseItem::NonDefault(Box::new(CaseItemNondefault { nodes: (a, b, c) })),
    ))
}

#[parser]
pub(crate) fn case_item_default(s: Span) -> IResult<Span, CaseItemDefault> {
    let (s, a) = keyword("default")(s)?;
    let (s, b) = opt(symbol(":"))(s)?;
    let (s, c) = statement_or_null(s)?;
    Ok((s, CaseItemDefault { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn case_pattern_item(s: Span) -> IResult<Span, CasePatternItem> {
    alt((
        case_pattern_item_nondefault,
        map(case_item_default, |x| CasePatternItem::Default(Box::new(x))),
    ))(s)
}

#[parser(MaybeRecursive)]
pub(crate) fn case_pattern_item_nondefault(s: Span) -> IResult<Span, CasePatternItem> {
    let (s, a) = pattern(s)?;
    let (s, b) = opt(pair(symbol("&&&"), expression))(s)?;
    let (s, c) = symbol(":")(s)?;
    let (s, d) = statement_or_null(s)?;
    Ok((
        s,
        CasePatternItem::NonDefault(Box::new(CasePatternItemNondefault {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser]
pub(crate) fn case_inside_item(s: Span) -> IResult<Span, CaseInsideItem> {
    alt((
        case_inside_item_nondefault,
        map(case_item_default, |x| CaseInsideItem::Default(Box::new(x))),
    ))(s)
}

#[parser(MaybeRecursive)]
pub(crate) fn case_inside_item_nondefault(s: Span) -> IResult<Span, CaseInsideItem> {
    let (s, a) = open_range_list(s)?;
    let (s, b) = symbol(":")(s)?;
    let (s, c) = statement_or_null(s)?;
    Ok((
        s,
        CaseInsideItem::NonDefault(Box::new(CaseInsideItemNondefault { nodes: (a, b, c) })),
    ))
}

#[parser]
pub(crate) fn case_item_expression(s: Span) -> IResult<Span, CaseItemExpression> {
    let (s, a) = expression(s)?;
    Ok((s, CaseItemExpression { nodes: (a,) }))
}

#[parser]
pub(crate) fn randcase_statement(s: Span) -> IResult<Span, RandcaseStatement> {
    let (s, a) = keyword("randcase")(s)?;
    let (s, b) = randcase_item(s)?;
    let (s, c) = many0(randcase_item)(s)?;
    let (s, d) = keyword("endcase")(s)?;
    Ok((
        s,
        RandcaseStatement {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser(MaybeRecursive)]
pub(crate) fn randcase_item(s: Span) -> IResult<Span, RandcaseItem> {
    let (s, a) = expression(s)?;
    let (s, b) = symbol(":")(s)?;
    let (s, c) = statement_or_null(s)?;
    Ok((s, RandcaseItem { nodes: (a, b, c) }))
}

#[parser(MaybeRecursive)]
pub(crate) fn open_range_list(s: Span) -> IResult<Span, OpenRangeList> {
    let (s, a) = list(symbol(","), open_value_range)(s)?;
    Ok((s, OpenRangeList { nodes: (a,) }))
}

#[parser]
pub(crate) fn open_value_range(s: Span) -> IResult<Span, OpenValueRange> {
    let (s, a) = value_range(s)?;
    Ok((s, OpenValueRange { nodes: (a,) }))
}
