use crate::*;

// -----------------------------------------------------------------------------

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn attribute_instance(s: Span) -> IResult<Span, AttributeInstance> {
    let (s, a) = symbol("(*")(s)?;
    let (s, b) = list(symbol(","), attr_spec)(s)?;
    let (s, c) = symbol("*)")(s)?;
    Ok((s, AttributeInstance { nodes: (a, b, c) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn attr_spec(s: Span) -> IResult<Span, AttrSpec> {
    let (s, a) = identifier(s)?;
    let (s, b) = opt(pair(symbol("="), constant_expression))(s)?;
    Ok((s, AttrSpec { nodes: (a, b) }))
}
