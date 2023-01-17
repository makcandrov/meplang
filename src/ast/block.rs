use bytes::Bytes;
use pest::iterators::Pair;
use primitive_types::H256;
use crate::parser::error::new_error_from_pair;
use crate::{ast::attribute::Attribute, parser::parser::FromPair};
use crate::parser::parser::Rule;

#[derive(Debug, Clone, Default)]
pub struct Block {
    name: String,
    attributes: Vec<Attribute>,
    lines: Vec<BlockLine>,
}

#[derive(Debug, Clone)]

pub enum BlockLine {
    String(String),
    Function { name: String, value: String},
    Bytes(Bytes),
}

impl FromPair for Block {
    fn from_pair(block_decl_with_attr: pest::iterators::Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_decl_with_attr.as_rule() == Rule::block_decl_with_attr);

        let mut res = Self::default();
        let mut seeking_attributes = true;
        for attr_or_block in block_decl_with_attr.into_inner() {
            match attr_or_block.as_rule() {
                Rule::attribute => {
                    assert!(seeking_attributes);
                },
                Rule::block_decl => {
                    seeking_attributes = false;
                    let mut block_decl_inner = attr_or_block.into_inner();

                    assert!(block_decl_inner.next().unwrap().as_rule() == Rule::block_keyword);

                    let block_name = block_decl_inner.next().unwrap();
                    assert!(block_name.as_rule() == Rule::var_name);
                    
                    res.name = block_name.as_str().to_owned();

                    while let Some(block_content) = block_decl_inner.next() {
                        assert!(block_content.as_rule() == Rule::block_content);
                        let mut block_content_inner = block_content.into_inner();
                        let block_line = block_content_inner.next().unwrap();

                        assert!(block_line.as_rule() == Rule::block_line);
                        assert!(block_content_inner.next() == None);

                        res.lines.push(BlockLine::from_pair(block_line)?);
                    }
                },
                _ => unreachable!(),
            }
        }
        Ok(res)
    }
}

impl FromPair for BlockLine {
    fn from_pair(block_line: Pair<Rule>) -> Result<Self, pest::error::Error<Rule>> {
        assert!(block_line.as_rule() == Rule::block_line);

        Ok(BlockLine::String(String::new()))
    }
}