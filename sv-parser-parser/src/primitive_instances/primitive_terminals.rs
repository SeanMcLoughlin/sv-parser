use crate::*;

// -----------------------------------------------------------------------------

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn enable_terminal(s: Span) -> IResult<Span, EnableTerminal> {
    let (s, a) = expression(s)?;
    Ok((s, EnableTerminal { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn inout_terminal(s: Span) -> IResult<Span, InoutTerminal> {
    let (s, a) = net_lvalue(s)?;
    Ok((s, InoutTerminal { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn input_terminal(s: Span) -> IResult<Span, InputTerminal> {
    let (s, a) = expression(s)?;
    Ok((s, InputTerminal { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn ncontrol_terminal(s: Span) -> IResult<Span, NcontrolTerminal> {
    let (s, a) = expression(s)?;
    Ok((s, NcontrolTerminal { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn output_terminal(s: Span) -> IResult<Span, OutputTerminal> {
    let (s, a) = net_lvalue(s)?;
    Ok((s, OutputTerminal { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn pcontrol_terminal(s: Span) -> IResult<Span, PcontrolTerminal> {
    let (s, a) = expression(s)?;
    Ok((s, PcontrolTerminal { nodes: (a,) }))
}
