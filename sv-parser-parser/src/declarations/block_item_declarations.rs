use crate::*;

// -----------------------------------------------------------------------------

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn block_item_declaration(s: Span) -> IResult<Span, BlockItemDeclaration> {
    alt((
        block_item_declaration_data,
        block_item_declaration_local_parameter,
        block_item_declaration_parameter,
        block_item_declaration_let,
    ))(s)
}

#[cfg_attr(feature = "recursive", recursive_parser)]
#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn block_item_declaration_data(s: Span) -> IResult<Span, BlockItemDeclaration> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = data_declaration(s)?;
    Ok((
        s,
        BlockItemDeclaration::Data(Box::new(BlockItemDeclarationData { nodes: (a, b) })),
    ))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn block_item_declaration_local_parameter(
    s: Span,
) -> IResult<Span, BlockItemDeclaration> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = local_parameter_declaration(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        BlockItemDeclaration::LocalParameter(Box::new(BlockItemDeclarationLocalParameter {
            nodes: (a, b, c),
        })),
    ))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn block_item_declaration_parameter(s: Span) -> IResult<Span, BlockItemDeclaration> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = parameter_declaration(s)?;
    let (s, c) = symbol(";")(s)?;
    Ok((
        s,
        BlockItemDeclaration::Parameter(Box::new(BlockItemDeclarationParameter {
            nodes: (a, b, c),
        })),
    ))
}

#[cfg_attr(feature = "trace", tracable_parser)]
#[cfg_attr(feature = "packrat", packrat_parser)]
pub(crate) fn block_item_declaration_let(s: Span) -> IResult<Span, BlockItemDeclaration> {
    let (s, a) = many0(attribute_instance)(s)?;
    let (s, b) = let_declaration(s)?;
    Ok((
        s,
        BlockItemDeclaration::Let(Box::new(BlockItemDeclarationLet { nodes: (a, b) })),
    ))
}
