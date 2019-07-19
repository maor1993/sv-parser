use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub enum ClockingDeclaration<'a> {
    Local(ClockingDeclarationLocal<'a>),
    Global(ClockingDeclarationGlobal<'a>),
}

#[derive(Debug, Node)]
pub struct ClockingDeclarationLocal<'a> {
    pub nodes: (
        Option<Default<'a>>,
        Keyword<'a>,
        Option<ClockingIdentifier<'a>>,
        ClockingEvent<'a>,
        Symbol<'a>,
        Vec<ClockingItem<'a>>,
        Keyword<'a>,
        Option<(Symbol<'a>, ClockingIdentifier<'a>)>,
    ),
}

#[derive(Debug, Node)]
pub struct Default<'a> {
    pub nodes: (Keyword<'a>,),
}

#[derive(Debug, Node)]
pub struct ClockingDeclarationGlobal<'a> {
    pub nodes: (
        Keyword<'a>,
        Keyword<'a>,
        Option<ClockingIdentifier<'a>>,
        ClockingEvent<'a>,
        Symbol<'a>,
        Keyword<'a>,
        Option<(Symbol<'a>, ClockingIdentifier<'a>)>,
    ),
}

#[derive(Debug, Node)]
pub enum ClockingEvent<'a> {
    Identifier(ClockingEventIdentifier<'a>),
    Expression(ClockingEventExpression<'a>),
}

#[derive(Debug, Node)]
pub struct ClockingEventIdentifier<'a> {
    pub nodes: (Symbol<'a>, Identifier<'a>),
}

#[derive(Debug, Node)]
pub struct ClockingEventExpression<'a> {
    pub nodes: (Symbol<'a>, Paren<'a, EventExpression<'a>>),
}

#[derive(Debug, Node)]
pub enum ClockingItem<'a> {
    Default(ClockingItemDefault<'a>),
    Direction(ClockingItemDirection<'a>),
    Assertion(ClockingItemAssertion<'a>),
}

#[derive(Debug, Node)]
pub struct ClockingItemDefault<'a> {
    pub nodes: (Keyword<'a>, DefaultSkew<'a>, Symbol<'a>),
}

#[derive(Debug, Node)]
pub struct ClockingItemDirection<'a> {
    pub nodes: (
        ClockingDirection<'a>,
        ListOfClockingDeclAssign<'a>,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct ClockingItemAssertion<'a> {
    pub nodes: (Vec<AttributeInstance<'a>>, AssertionItemDeclaration<'a>),
}

#[derive(Debug, Node)]
pub enum DefaultSkew<'a> {
    Input(DefaultSkewInput<'a>),
    Output(DefaultSkewOutput<'a>),
    InputOutput(DefaultSkewInputOutput<'a>),
}

#[derive(Debug, Node)]
pub struct DefaultSkewInput<'a> {
    pub nodes: (Keyword<'a>, ClockingSkew<'a>),
}

#[derive(Debug, Node)]
pub struct DefaultSkewOutput<'a> {
    pub nodes: (Keyword<'a>, ClockingSkew<'a>),
}

#[derive(Debug, Node)]
pub struct DefaultSkewInputOutput<'a> {
    pub nodes: (Keyword<'a>, ClockingSkew<'a>, Keyword<'a>, ClockingSkew<'a>),
}

#[derive(Debug, Node)]
pub enum ClockingDirection<'a> {
    Input(ClockingDirectionInput<'a>),
    Output(ClockingDirectionOutput<'a>),
    InputOutput(ClockingDirectionInputOutput<'a>),
    Inout(Keyword<'a>),
}

#[derive(Debug, Node)]
pub struct ClockingDirectionInput<'a> {
    pub nodes: (Keyword<'a>, Option<ClockingSkew<'a>>),
}

#[derive(Debug, Node)]
pub struct ClockingDirectionOutput<'a> {
    pub nodes: (Keyword<'a>, Option<ClockingSkew<'a>>),
}

#[derive(Debug, Node)]
pub struct ClockingDirectionInputOutput<'a> {
    pub nodes: (
        Keyword<'a>,
        Option<ClockingSkew<'a>>,
        Keyword<'a>,
        Option<ClockingSkew<'a>>,
    ),
}

#[derive(Debug, Node)]
pub struct ListOfClockingDeclAssign<'a> {
    pub nodes: (List<Symbol<'a>, ClockingDeclAssign<'a>>,),
}

#[derive(Debug, Node)]
pub struct ClockingDeclAssign<'a> {
    pub nodes: (SignalIdentifier<'a>, Option<(Symbol<'a>, Expression<'a>)>),
}

#[derive(Debug, Node)]
pub enum ClockingSkew<'a> {
    Edge(ClockingSkewEdge<'a>),
    DelayControl(DelayControl<'a>),
}

#[derive(Debug, Node)]
pub struct ClockingSkewEdge<'a> {
    pub nodes: (EdgeIdentifier<'a>, Option<DelayControl<'a>>),
}

#[derive(Debug, Node)]
pub struct ClockingDrive<'a> {
    pub nodes: (
        ClockvarExpression<'a>,
        Symbol<'a>,
        Option<CycleDelay<'a>>,
        Expression<'a>,
    ),
}

#[derive(Debug, Node)]
pub enum CycleDelay<'a> {
    Integral(CycleDelayIntegral<'a>),
    Identifier(CycleDelayIdentifier<'a>),
    Expression(CycleDelayExpression<'a>),
}

#[derive(Debug, Node)]
pub struct CycleDelayIntegral<'a> {
    pub nodes: (Symbol<'a>, IntegralNumber<'a>),
}

#[derive(Debug, Node)]
pub struct CycleDelayIdentifier<'a> {
    pub nodes: (Symbol<'a>, Identifier<'a>),
}

#[derive(Debug, Node)]
pub struct CycleDelayExpression<'a> {
    pub nodes: (Symbol<'a>, Paren<'a, Expression<'a>>),
}

#[derive(Debug, Node)]
pub struct Clockvar<'a> {
    pub nodes: (HierarchicalIdentifier<'a>,),
}

#[derive(Debug, Node)]
pub struct ClockvarExpression<'a> {
    pub nodes: (Clockvar<'a>, Select<'a>),
}
// -----------------------------------------------------------------------------

#[parser]
pub fn clocking_declaration(s: Span) -> IResult<Span, ClockingDeclaration> {
    alt((clocking_declaration_local, clocking_declaration_global))(s)
}

#[parser]
pub fn clocking_declaration_local(s: Span) -> IResult<Span, ClockingDeclaration> {
    let (s, a) = opt(default)(s)?;
    let (s, b) = keyword("clocking")(s)?;
    let (s, c) = opt(clocking_identifier)(s)?;
    let (s, d) = clocking_event(s)?;
    let (s, e) = symbol(";")(s)?;
    let (s, f) = many0(clocking_item)(s)?;
    let (s, g) = keyword("endclocking")(s)?;
    let (s, h) = opt(pair(symbol(":"), clocking_identifier))(s)?;
    Ok((
        s,
        ClockingDeclaration::Local(ClockingDeclarationLocal {
            nodes: (a, b, c, d, e, f, g, h),
        }),
    ))
}

#[parser]
pub fn default(s: Span) -> IResult<Span, Default> {
    let (s, a) = keyword("default")(s)?;
    Ok((s, Default { nodes: (a,) }))
}

#[parser]
pub fn clocking_declaration_global(s: Span) -> IResult<Span, ClockingDeclaration> {
    let (s, a) = keyword("global")(s)?;
    let (s, b) = keyword("clocking")(s)?;
    let (s, c) = opt(clocking_identifier)(s)?;
    let (s, d) = clocking_event(s)?;
    let (s, e) = symbol(";")(s)?;
    let (s, f) = keyword("endclocking")(s)?;
    let (s, g) = opt(pair(symbol(":"), clocking_identifier))(s)?;
    Ok((
        s,
        ClockingDeclaration::Global(ClockingDeclarationGlobal {
            nodes: (a, b, c, d, e, f, g),
        }),
    ))
}

#[parser]
pub fn clocking_event(s: Span) -> IResult<Span, ClockingEvent> {
    alt((clocking_event_identifier, clocking_event_expression))(s)
}

#[parser]
pub fn clocking_event_identifier(s: Span) -> IResult<Span, ClockingEvent> {
    let (s, a) = symbol("@")(s)?;
    let (s, b) = identifier(s)?;
    Ok((
        s,
        ClockingEvent::Identifier(ClockingEventIdentifier { nodes: (a, b) }),
    ))
}

#[parser]
pub fn clocking_event_expression(s: Span) -> IResult<Span, ClockingEvent> {
    let (s, a) = symbol("@")(s)?;
    let (s, b) = paren(event_expression)(s)?;
    Ok((
        s,
        ClockingEvent::Expression(ClockingEventExpression { nodes: (a, b) }),
    ))
}

#[parser]
pub fn clocking_item(s: Span) -> IResult<Span, ClockingItem> {
    alt((
        clocking_item_default,
        clocking_item_direction,
        clocking_item_assertion,
    ))(s)
}

#[parser]
pub fn clocking_item_default(s: Span) -> IResult<Span, ClockingItem> {
    let (s, a) = keyword("default")(s)?;
    let (s, b) = default_skew(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        ClockingItem::Default(ClockingItemDefault { nodes: (a, b, c) }),
    ))
}

#[parser]
pub fn clocking_item_direction(s: Span) -> IResult<Span, ClockingItem> {
    let (s, a) = clocking_direction(s)?;
    let (s, b) = list_of_clocking_decl_assign(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        ClockingItem::Direction(ClockingItemDirection { nodes: (a, b, c) }),
    ))
}

#[parser]
pub fn clocking_item_assertion(s: Span) -> IResult<Span, ClockingItem> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = assertion_item_declaration(s)?;
    Ok((
        s,
        ClockingItem::Assertion(ClockingItemAssertion { nodes: (a, b) }),
    ))
}

#[parser]
pub fn default_skew(s: Span) -> IResult<Span, DefaultSkew> {
    alt((
        default_skew_input,
        default_skew_output,
        default_skew_input_output,
    ))(s)
}

#[parser]
pub fn default_skew_input(s: Span) -> IResult<Span, DefaultSkew> {
    let (s, a) = keyword("input")(s)?;
    let (s, b) = clocking_skew(s)?;
    Ok((s, DefaultSkew::Input(DefaultSkewInput { nodes: (a, b) })))
}

#[parser]
pub fn default_skew_output(s: Span) -> IResult<Span, DefaultSkew> {
    let (s, a) = keyword("output")(s)?;
    let (s, b) = clocking_skew(s)?;
    Ok((s, DefaultSkew::Output(DefaultSkewOutput { nodes: (a, b) })))
}

#[parser]
pub fn default_skew_input_output(s: Span) -> IResult<Span, DefaultSkew> {
    let (s, a) = keyword("input")(s)?;
    let (s, b) = clocking_skew(s)?;
    let (s, c) = keyword("output")(s)?;
    let (s, d) = clocking_skew(s)?;
    Ok((
        s,
        DefaultSkew::InputOutput(DefaultSkewInputOutput {
            nodes: (a, b, c, d),
        }),
    ))
}

#[parser]
pub fn clocking_direction(s: Span) -> IResult<Span, ClockingDirection> {
    alt((
        clocking_direction_input,
        clocking_direction_output,
        clocking_direction_input_output,
        clocking_direction_inout,
    ))(s)
}

#[parser]
pub fn clocking_direction_input(s: Span) -> IResult<Span, ClockingDirection> {
    let (s, a) = keyword("input")(s)?;
    let (s, b) = opt(clocking_skew)(s)?;
    Ok((
        s,
        ClockingDirection::Input(ClockingDirectionInput { nodes: (a, b) }),
    ))
}

#[parser]
pub fn clocking_direction_output(s: Span) -> IResult<Span, ClockingDirection> {
    let (s, a) = keyword("output")(s)?;
    let (s, b) = opt(clocking_skew)(s)?;
    Ok((
        s,
        ClockingDirection::Output(ClockingDirectionOutput { nodes: (a, b) }),
    ))
}

#[parser]
pub fn clocking_direction_input_output(s: Span) -> IResult<Span, ClockingDirection> {
    let (s, a) = keyword("input")(s)?;
    let (s, b) = opt(clocking_skew)(s)?;
    let (s, c) = keyword("output")(s)?;
    let (s, d) = opt(clocking_skew)(s)?;
    Ok((
        s,
        ClockingDirection::InputOutput(ClockingDirectionInputOutput {
            nodes: (a, b, c, d),
        }),
    ))
}

#[parser]
pub fn clocking_direction_inout(s: Span) -> IResult<Span, ClockingDirection> {
    let (s, a) = keyword("inout")(s)?;
    Ok((s, ClockingDirection::Inout(a)))
}

#[parser]
pub fn list_of_clocking_decl_assign(s: Span) -> IResult<Span, ListOfClockingDeclAssign> {
    let (s, a) = list(symbol(","), clocking_decl_assign)(s)?;
    Ok((s, ListOfClockingDeclAssign { nodes: (a,) }))
}

#[parser]
pub fn clocking_decl_assign(s: Span) -> IResult<Span, ClockingDeclAssign> {
    let (s, a) = signal_identifier(s)?;
    let (s, b) = opt(pair(symbol("="), expression))(s)?;
    Ok((s, ClockingDeclAssign { nodes: (a, b) }))
}

#[parser]
pub fn clocking_skew(s: Span) -> IResult<Span, ClockingSkew> {
    alt((
        clocking_skew_edge,
        map(delay_control, |x| ClockingSkew::DelayControl(x)),
    ))(s)
}

#[parser]
pub fn clocking_skew_edge(s: Span) -> IResult<Span, ClockingSkew> {
    let (s, a) = edge_identifier(s)?;
    let (s, b) = opt(delay_control)(s)?;
    Ok((s, ClockingSkew::Edge(ClockingSkewEdge { nodes: (a, b) })))
}

#[parser]
pub fn clocking_drive(s: Span) -> IResult<Span, ClockingDrive> {
    let (s, a) = clockvar_expression(s)?;
    let (s, b) = symbol("<=")(s)?;
    let (s, c) = opt(cycle_delay)(s)?;
    let (s, d) = expression(s)?;
    Ok((
        s,
        ClockingDrive {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub fn cycle_delay(s: Span) -> IResult<Span, CycleDelay> {
    alt((
        cycle_delay_integral,
        cycle_delay_identifier,
        cycle_delay_expression,
    ))(s)
}

#[parser]
pub fn cycle_delay_integral(s: Span) -> IResult<Span, CycleDelay> {
    let (s, a) = symbol("##")(s)?;
    let (s, b) = integral_number(s)?;
    Ok((
        s,
        CycleDelay::Integral(CycleDelayIntegral { nodes: (a, b) }),
    ))
}

#[parser]
pub fn cycle_delay_identifier(s: Span) -> IResult<Span, CycleDelay> {
    let (s, a) = symbol("##")(s)?;
    let (s, b) = identifier(s)?;
    Ok((
        s,
        CycleDelay::Identifier(CycleDelayIdentifier { nodes: (a, b) }),
    ))
}

#[parser]
pub fn cycle_delay_expression(s: Span) -> IResult<Span, CycleDelay> {
    let (s, a) = symbol("##")(s)?;
    let (s, b) = paren(expression)(s)?;
    Ok((
        s,
        CycleDelay::Expression(CycleDelayExpression { nodes: (a, b) }),
    ))
}

#[parser]
pub fn clockvar(s: Span) -> IResult<Span, Clockvar> {
    let (s, a) = hierarchical_identifier(s)?;
    Ok((s, Clockvar { nodes: (a,) }))
}

#[parser]
pub fn clockvar_expression(s: Span) -> IResult<Span, ClockvarExpression> {
    let (s, a) = clockvar(s)?;
    let (s, b) = select(s)?;
    Ok((s, ClockvarExpression { nodes: (a, b) }))
}

// -----------------------------------------------------------------------------
