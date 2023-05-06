use syn::{
    parse::{Parse, ParseStream},
    Expr, Result, Token,
};

#[derive(Debug)]
pub struct Expressions(pub Vec<Expr>);

impl Parse for Expressions {
    fn parse(input: ParseStream) -> Result<Self> {
        let expressions: Vec<_> = input
            .parse_terminated(Expr::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(Expressions(expressions))
    }
}
