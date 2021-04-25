use crate::*;

// -----------------------------------------------------------------------------

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn program_instantiation(s: Span) -> IResult<Span, ProgramInstantiation> {
    let (s, a) = program_identifier(s)?;
    let (s, b) = opt(parameter_value_assignment)(s)?;
    let (s, c) = list(symbol(","), hierarchical_instance)(s)?;
    let (s, d) = symbol(";")(s)?;
    Ok((
        s,
        ProgramInstantiation {
            nodes: (a, b, c, d),
        },
    ))
}
