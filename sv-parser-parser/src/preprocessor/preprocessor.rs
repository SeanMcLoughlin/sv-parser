use crate::*;

// -----------------------------------------------------------------------------

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn preprocessor_text(s: Span) -> IResult<Span, PreprocessorText> {
    let (s, a) = many0(source_description)(s)?;
    Ok((s, PreprocessorText { nodes: (a,) }))
}
