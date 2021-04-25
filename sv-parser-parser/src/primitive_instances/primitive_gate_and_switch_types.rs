use crate::*;

// -----------------------------------------------------------------------------

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn cmos_switchtype(s: Span) -> IResult<Span, CmosSwitchtype> {
    let (s, a) = alt((keyword("cmos"), keyword("rcmos")))(s)?;
    Ok((s, CmosSwitchtype { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn enable_gatetype(s: Span) -> IResult<Span, EnableGatetype> {
    let (s, a) = alt((
        keyword("bufif0"),
        keyword("bufif1"),
        keyword("notif0"),
        keyword("notif1"),
    ))(s)?;
    Ok((s, EnableGatetype { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn mos_switchtype(s: Span) -> IResult<Span, MosSwitchtype> {
    let (s, a) = alt((
        keyword("nmos"),
        keyword("pmos"),
        keyword("rnmos"),
        keyword("rpmos"),
    ))(s)?;
    Ok((s, MosSwitchtype { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn n_input_gatetype(s: Span) -> IResult<Span, NInputGatetype> {
    let (s, a) = alt((
        keyword("and"),
        keyword("nand"),
        keyword("or"),
        keyword("nor"),
        keyword("xor"),
        keyword("xnor"),
    ))(s)?;
    Ok((s, NInputGatetype { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn n_output_gatetype(s: Span) -> IResult<Span, NOutputGatetype> {
    let (s, a) = alt((keyword("buf"), keyword("not")))(s)?;
    Ok((s, NOutputGatetype { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn pass_en_switchtype(s: Span) -> IResult<Span, PassEnSwitchtype> {
    let (s, a) = alt((
        keyword("tranif0"),
        keyword("tranif1"),
        keyword("rtranif0"),
        keyword("rtranif1"),
    ))(s)?;
    Ok((s, PassEnSwitchtype { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn pass_switchtype(s: Span) -> IResult<Span, PassSwitchtype> {
    let (s, a) = alt((keyword("tran"), keyword("rtran")))(s)?;
    Ok((s, PassSwitchtype { nodes: (a,) }))
}
