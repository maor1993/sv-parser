use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::multi::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Node)]
pub struct ModuleInstantiation {
    pub nodes: (
        ModuleIdentifier,
        Option<ParameterValueAssignment>,
        List<Symbol, HierarchicalInstance>,
        Symbol,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct ParameterValueAssignment {
    pub nodes: (Symbol, Paren<Option<ListOfParameterAssignments>>),
}

#[derive(Clone, Debug, Node)]
pub enum ListOfParameterAssignments {
    Ordered(Box<ListOfParameterAssignmentsOrdered>),
    Named(Box<ListOfParameterAssignmentsNamed>),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfParameterAssignmentsOrdered {
    pub nodes: (List<Symbol, OrderedParameterAssignment>,),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfParameterAssignmentsNamed {
    pub nodes: (List<Symbol, NamedParameterAssignment>,),
}

#[derive(Clone, Debug, Node)]
pub struct OrderedParameterAssignment {
    pub nodes: (ParamExpression,),
}

#[derive(Clone, Debug, Node)]
pub struct NamedParameterAssignment {
    pub nodes: (Symbol, ParameterIdentifier, Paren<Option<ParamExpression>>),
}

#[derive(Clone, Debug, Node)]
pub struct HierarchicalInstance {
    pub nodes: (NameOfInstance, Paren<Option<ListOfPortConnections>>),
}

#[derive(Clone, Debug, Node)]
pub struct NameOfInstance {
    pub nodes: (InstanceIdentifier, Vec<UnpackedDimension>),
}

#[derive(Clone, Debug, Node)]
pub enum ListOfPortConnections {
    Ordered(Box<ListOfPortConnectionsOrdered>),
    Named(Box<ListOfPortConnectionsNamed>),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfPortConnectionsOrdered {
    pub nodes: (List<Symbol, OrderedPortConnection>,),
}

#[derive(Clone, Debug, Node)]
pub struct ListOfPortConnectionsNamed {
    pub nodes: (List<Symbol, NamedPortConnection>,),
}

#[derive(Clone, Debug, Node)]
pub struct OrderedPortConnection {
    pub nodes: (Vec<AttributeInstance>, Option<Expression>),
}

#[derive(Clone, Debug, Node)]
pub enum NamedPortConnection {
    Identifier(Box<NamedPortConnectionIdentifier>),
    Asterisk(Box<NamedPortConnectionAsterisk>),
}

#[derive(Clone, Debug, Node)]
pub struct NamedPortConnectionIdentifier {
    pub nodes: (
        Vec<AttributeInstance>,
        Symbol,
        PortIdentifier,
        Option<Paren<Option<Expression>>>,
    ),
}

#[derive(Clone, Debug, Node)]
pub struct NamedPortConnectionAsterisk {
    pub nodes: (Vec<AttributeInstance>, Symbol),
}

// -----------------------------------------------------------------------------

#[parser]
pub(crate) fn module_instantiation(s: Span) -> IResult<Span, ModuleInstantiation> {
    let (s, a) = module_identifier(s)?;
    let (s, b) = opt(parameter_value_assignment)(s)?;
    let (s, c) = list(symbol(","), hierarchical_instance)(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        ModuleInstantiation {
            nodes: (a, b, c, d),
        },
    ))
}

#[parser]
pub(crate) fn parameter_value_assignment(s: Span) -> IResult<Span, ParameterValueAssignment> {
    let (s, a) = symbol("#")(s)?;
    let (s, b) = paren(opt(list_of_parameter_assignments))(s)?;
    Ok((s, ParameterValueAssignment { nodes: (a, b) }))
}

#[parser]
pub(crate) fn list_of_parameter_assignments(s: Span) -> IResult<Span, ListOfParameterAssignments> {
    alt((
        list_of_parameter_assignments_ordered,
        list_of_parameter_assignments_named,
    ))(s)
}

#[parser(MaybeRecursive)]
pub(crate) fn list_of_parameter_assignments_ordered(s: Span) -> IResult<Span, ListOfParameterAssignments> {
    let (s, a) = list(symbol(","), ordered_parameter_assignment)(s)?;
    Ok((
        s,
        ListOfParameterAssignments::Ordered(Box::new(ListOfParameterAssignmentsOrdered {
            nodes: (a,),
        })),
    ))
}

#[parser]
pub(crate) fn list_of_parameter_assignments_named(s: Span) -> IResult<Span, ListOfParameterAssignments> {
    let (s, a) = list(symbol(","), named_parameter_assignment)(s)?;
    Ok((
        s,
        ListOfParameterAssignments::Named(Box::new(ListOfParameterAssignmentsNamed {
            nodes: (a,),
        })),
    ))
}

#[parser]
pub(crate) fn ordered_parameter_assignment(s: Span) -> IResult<Span, OrderedParameterAssignment> {
    let (s, x) = param_expression(s)?;
    Ok((s, OrderedParameterAssignment { nodes: (x,) }))
}

#[parser]
pub(crate) fn named_parameter_assignment(s: Span) -> IResult<Span, NamedParameterAssignment> {
    let (s, a) = symbol(".")(s)?;
    let (s, b) = parameter_identifier(s)?;
    let (s, c) = paren(opt(param_expression))(s)?;
    Ok((s, NamedParameterAssignment { nodes: (a, b, c) }))
}

#[parser]
pub(crate) fn hierarchical_instance(s: Span) -> IResult<Span, HierarchicalInstance> {
    let (s, a) = name_of_instance(s)?;
    let (s, b) = paren(opt(list_of_port_connections))(s)?;
    Ok((s, HierarchicalInstance { nodes: (a, b) }))
}

#[parser]
pub(crate) fn name_of_instance(s: Span) -> IResult<Span, NameOfInstance> {
    let (s, x) = instance_identifier(s)?;
    let (s, y) = many0(unpacked_dimension)(s)?;
    Ok((s, NameOfInstance { nodes: (x, y) }))
}

#[parser]
pub(crate) fn list_of_port_connections(s: Span) -> IResult<Span, ListOfPortConnections> {
    alt((
        list_of_port_connections_ordered,
        list_of_port_connections_named,
    ))(s)
}

#[parser(MaybeRecursive)]
pub(crate) fn list_of_port_connections_ordered(s: Span) -> IResult<Span, ListOfPortConnections> {
    let (s, a) = list(symbol(","), ordered_port_connection)(s)?;
    Ok((
        s,
        ListOfPortConnections::Ordered(Box::new(ListOfPortConnectionsOrdered { nodes: (a,) })),
    ))
}

#[parser]
pub(crate) fn list_of_port_connections_named(s: Span) -> IResult<Span, ListOfPortConnections> {
    let (s, a) = list(symbol(","), named_port_connection)(s)?;
    Ok((
        s,
        ListOfPortConnections::Named(Box::new(ListOfPortConnectionsNamed { nodes: (a,) })),
    ))
}

#[parser(MaybeRecursive)]
pub(crate) fn ordered_port_connection(s: Span) -> IResult<Span, OrderedPortConnection> {
    let (s, x) = many0(attribute_instance)(s)?;
    let (s, y) = opt(expression)(s)?;
    Ok((s, OrderedPortConnection { nodes: (x, y) }))
}

#[parser]
pub(crate) fn named_port_connection(s: Span) -> IResult<Span, NamedPortConnection> {
    alt((
        named_port_connection_identifier,
        named_port_connection_asterisk,
    ))(s)
}

#[parser]
pub(crate) fn named_port_connection_identifier(s: Span) -> IResult<Span, NamedPortConnection> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = symbol(".")(s)?;
    let (s, c) = port_identifier(s)?;
    let (s, d) = opt(paren(opt(expression)))(s)?;
    Ok((
        s,
        NamedPortConnection::Identifier(Box::new(NamedPortConnectionIdentifier {
            nodes: (a, b, c, d),
        })),
    ))
}

#[parser]
pub(crate) fn named_port_connection_asterisk(s: Span) -> IResult<Span, NamedPortConnection> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = symbol(".*")(s)?;
    Ok((
        s,
        NamedPortConnection::Asterisk(Box::new(NamedPortConnectionAsterisk { nodes: (a, b) })),
    ))
}

// -----------------------------------------------------------------------------
