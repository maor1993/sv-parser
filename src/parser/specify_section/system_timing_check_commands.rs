use crate::ast::*;
use crate::parser::*;
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

// -----------------------------------------------------------------------------

#[derive(Debug, Node)]
pub enum SystemTimingCheck<'a> {
    SetupTimingCheck(SetupTimingCheck<'a>),
    HoldTimingCheck(HoldTimingCheck<'a>),
    SetupholdTimingCheck(SetupholdTimingCheck<'a>),
    RecoveryTimingCheck(RecoveryTimingCheck<'a>),
    RemovalTimingCheck(RemovalTimingCheck<'a>),
    RecremTimingCheck(RecremTimingCheck<'a>),
    SkewTimingCheck(SkewTimingCheck<'a>),
    TimeskewTimingCheck(TimeskewTimingCheck<'a>),
    FullskewTimingCheck(FullskewTimingCheck<'a>),
    PeriodTimingCheck(PeriodTimingCheck<'a>),
    WidthTimingCheck(WidthTimingCheck<'a>),
    NochargeTimingCheck(NochargeTimingCheck<'a>),
}

#[derive(Debug, Node)]
pub struct SetupTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                DataEvent<'a>,
                Symbol<'a>,
                ReferenceEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct HoldTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct SetupholdTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(
                    Symbol<'a>,
                    Option<Notifier<'a>>,
                    Option<(
                        Symbol<'a>,
                        Option<TimestampCondition<'a>>,
                        Option<(
                            Symbol<'a>,
                            Option<TimecheckCondition<'a>>,
                            Option<(
                                Symbol<'a>,
                                Option<DelayedReference<'a>>,
                                Option<(Symbol<'a>, Option<DelayedData<'a>>)>,
                            )>,
                        )>,
                    )>,
                )>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct RecoveryTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct RemovalTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct RecremTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(
                    Symbol<'a>,
                    Option<Notifier<'a>>,
                    Option<(
                        Symbol<'a>,
                        Option<TimestampCondition<'a>>,
                        Option<(
                            Symbol<'a>,
                            Option<TimecheckCondition<'a>>,
                            Option<(
                                Symbol<'a>,
                                Option<DelayedReference<'a>>,
                                Option<(Symbol<'a>, Option<DelayedData<'a>>)>,
                            )>,
                        )>,
                    )>,
                )>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct SkewTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct TimeskewTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(
                    Symbol<'a>,
                    Option<Notifier<'a>>,
                    Option<(
                        Symbol<'a>,
                        Option<EventBasedFlag<'a>>,
                        Option<(Symbol<'a>, Option<RemainActiveFlag<'a>>)>,
                    )>,
                )>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct FullskewTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(
                    Symbol<'a>,
                    Option<Notifier<'a>>,
                    Option<(
                        Symbol<'a>,
                        Option<EventBasedFlag<'a>>,
                        Option<(Symbol<'a>, Option<RemainActiveFlag<'a>>)>,
                    )>,
                )>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct PeriodTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ControlledReferenceEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct WidthTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ControlledReferenceEvent<'a>,
                Symbol<'a>,
                TimingCheckLimit<'a>,
                Symbol<'a>,
                Threshold<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

#[derive(Debug, Node)]
pub struct NochargeTimingCheck<'a> {
    pub nodes: (
        Keyword<'a>,
        Paren<
            'a,
            (
                ReferenceEvent<'a>,
                Symbol<'a>,
                DataEvent<'a>,
                Symbol<'a>,
                StartEdgeOffset<'a>,
                Symbol<'a>,
                EndEdgeOffset<'a>,
                Option<(Symbol<'a>, Option<Notifier<'a>>)>,
            ),
        >,
        Symbol<'a>,
    ),
}

// -----------------------------------------------------------------------------

#[parser]
pub fn system_timing_check(s: Span) -> IResult<Span, SystemTimingCheck> {
    alt((
        map(setup_timing_check, |x| {
            SystemTimingCheck::SetupTimingCheck(x)
        }),
        map(hold_timing_check, |x| SystemTimingCheck::HoldTimingCheck(x)),
        map(setuphold_timing_check, |x| {
            SystemTimingCheck::SetupholdTimingCheck(x)
        }),
        map(recovery_timing_check, |x| {
            SystemTimingCheck::RecoveryTimingCheck(x)
        }),
        map(removal_timing_check, |x| {
            SystemTimingCheck::RemovalTimingCheck(x)
        }),
        map(recrem_timing_check, |x| {
            SystemTimingCheck::RecremTimingCheck(x)
        }),
        map(skew_timing_check, |x| SystemTimingCheck::SkewTimingCheck(x)),
        map(timeskew_timing_check, |x| {
            SystemTimingCheck::TimeskewTimingCheck(x)
        }),
        map(fullskew_timing_check, |x| {
            SystemTimingCheck::FullskewTimingCheck(x)
        }),
        map(period_timing_check, |x| {
            SystemTimingCheck::PeriodTimingCheck(x)
        }),
        map(width_timing_check, |x| {
            SystemTimingCheck::WidthTimingCheck(x)
        }),
        map(nocharge_timing_check, |x| {
            SystemTimingCheck::NochargeTimingCheck(x)
        }),
    ))(s)
}

#[parser]
pub fn setup_timing_check(s: Span) -> IResult<Span, SetupTimingCheck> {
    let (s, a) = keyword("$setup")(s)?;
    let (s, b) = paren(tuple((
        data_event,
        symbol(","),
        referecne_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, SetupTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn hold_timing_check(s: Span) -> IResult<Span, HoldTimingCheck> {
    let (s, a) = keyword("$setup")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, HoldTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn setuphold_timing_check(s: Span) -> IResult<Span, SetupholdTimingCheck> {
    let (s, a) = keyword("$setuphold")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(timestamp_condition),
                opt(triple(
                    symbol(","),
                    opt(timecheck_condition),
                    opt(triple(
                        symbol(","),
                        opt(delayed_reference),
                        opt(pair(symbol(","), opt(delayed_data))),
                    )),
                )),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, SetupholdTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn recovery_timing_check(s: Span) -> IResult<Span, RecoveryTimingCheck> {
    let (s, a) = keyword("$recovery")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, RecoveryTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn removal_timing_check(s: Span) -> IResult<Span, RemovalTimingCheck> {
    let (s, a) = keyword("$removal")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, RemovalTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn recrem_timing_check(s: Span) -> IResult<Span, RecremTimingCheck> {
    let (s, a) = keyword("$recrem")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(timestamp_condition),
                opt(triple(
                    symbol(","),
                    opt(timecheck_condition),
                    opt(triple(
                        symbol(","),
                        opt(delayed_reference),
                        opt(pair(symbol(","), opt(delayed_data))),
                    )),
                )),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, RecremTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn skew_timing_check(s: Span) -> IResult<Span, SkewTimingCheck> {
    let (s, a) = keyword("$skew")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, SkewTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn timeskew_timing_check(s: Span) -> IResult<Span, TimeskewTimingCheck> {
    let (s, a) = keyword("$timeskew")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(event_based_flag),
                opt(pair(symbol(","), opt(remain_active_flag))),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, TimeskewTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn fullskew_timing_check(s: Span) -> IResult<Span, FullskewTimingCheck> {
    let (s, a) = keyword("$fullskew")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        timing_check_limit,
        opt(triple(
            symbol(","),
            opt(notifier),
            opt(triple(
                symbol(","),
                opt(event_based_flag),
                opt(pair(symbol(","), opt(remain_active_flag))),
            )),
        )),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, FullskewTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn period_timing_check(s: Span) -> IResult<Span, PeriodTimingCheck> {
    let (s, a) = keyword("$period")(s)?;
    let (s, b) = paren(tuple((
        controlled_referecne_event,
        symbol(","),
        timing_check_limit,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, PeriodTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn width_timing_check(s: Span) -> IResult<Span, WidthTimingCheck> {
    let (s, a) = keyword("$width")(s)?;
    let (s, b) = paren(tuple((
        controlled_referecne_event,
        symbol(","),
        timing_check_limit,
        symbol(","),
        threshold,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, WidthTimingCheck { nodes: (a, b, c) }))
}

#[parser]
pub fn nocharge_timing_check(s: Span) -> IResult<Span, NochargeTimingCheck> {
    let (s, a) = keyword("$nocharge")(s)?;
    let (s, b) = paren(tuple((
        referecne_event,
        symbol(","),
        data_event,
        symbol(","),
        start_edge_offset,
        symbol(","),
        end_edge_offset,
        opt(pair(symbol(","), opt(notifier))),
    )))(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((s, NochargeTimingCheck { nodes: (a, b, c) }))
}
