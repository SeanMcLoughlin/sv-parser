use crate::*;

// -----------------------------------------------------------------------------

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn unary_operator(s: Span) -> IResult<Span, UnaryOperator> {
    let (s, a) = alt((
        symbol("+"),
        symbol("-"),
        symbol("!"),
        symbol("&"),
        symbol("|"),
        symbol("~&"),
        symbol("~|"),
        symbol("~^"),
        symbol("^~"),
        symbol("^"),
        symbol("~"),
    ))(s)?;
    Ok((s, UnaryOperator { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn binary_operator(s: Span) -> IResult<Span, BinaryOperator> {
    let (s, a) = alt((
        alt((
            symbol("+"),
            symbol("->"),
            symbol("-"),
            symbol("**"),
            symbol("*"),
            symbol("/"),
            symbol("%"),
            symbol("==="),
            symbol("==?"),
            symbol("=="),
            symbol("!=="),
            symbol("!=?"),
            symbol("!="),
            symbol("&&"),
            symbol("||"),
        )),
        alt((
            symbol("&"),
            symbol("|"),
            symbol("^~"),
            symbol("^"),
            symbol("~^"),
            symbol(">>>"),
            symbol(">>"),
            symbol("<<<"),
            symbol("<<"),
            symbol("<->"),
            symbol("<="),
            symbol("<"),
            symbol(">="),
            symbol(">"),
        )),
    ))(s)?;
    Ok((s, BinaryOperator { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn inc_or_dec_operator(s: Span) -> IResult<Span, IncOrDecOperator> {
    let (s, a) = alt((symbol("++"), symbol("--")))(s)?;
    Ok((s, IncOrDecOperator { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn unary_module_path_operator(s: Span) -> IResult<Span, UnaryModulePathOperator> {
    let (s, a) = alt((
        symbol("!"),
        symbol("&"),
        symbol("|"),
        symbol("~&"),
        symbol("~|"),
        symbol("~^"),
        symbol("^~"),
        symbol("^"),
        symbol("~"),
    ))(s)?;
    Ok((s, UnaryModulePathOperator { nodes: (a,) }))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn binary_module_path_operator(s: Span) -> IResult<Span, BinaryModulePathOperator> {
    let (s, a) = alt((
        symbol("=="),
        symbol("!="),
        symbol("&&"),
        symbol("||"),
        symbol("&"),
        symbol("|"),
        symbol("^~"),
        symbol("^"),
        symbol("~^"),
    ))(s)?;
    Ok((s, BinaryModulePathOperator { nodes: (a,) }))
}
