use pest::iterators::Pair;
use primitive_types::H256;
use crate::parser::parser::Rule;

#[derive(Default, Debug, Clone)]
pub struct Expression {
    pub name: String,
    pub value: H256,
}

impl Expression {
    pub fn from_pair(expr: Pair<Rule>) -> Self {
        assert!(expr.as_rule() == Rule::expression);

        let mut expr_inner = expr.into_inner();
        let (name, value) = (expr_inner.next().unwrap(), expr_inner.next().unwrap());

        assert!(expr_inner.next() == None);
        assert!(name.as_rule() == Rule::var_name);
        assert!(value.as_rule() == Rule::hex_litteral);

        Self {
            name: name.as_str().to_owned(),
            value: H256::default(),
        }
    }
}
