use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    variable_compiler::VariableCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{bail, Result};
use grit_pattern_matcher::pattern::{CallBuiltIn, Pattern};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;
pub(crate) struct StripQuoteCompiler;

impl NodeCompiler for StripQuoteCompiler {
    type TargetPattern = CallBuiltIn<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let variable_node = node.child_by_field_name("variable");
        let variable = variable_node
            .map(
                |n| match (n.text(), VariableCompiler::from_node(&n, context)) {
                    (Ok(name), Ok(variable)) => Ok((name.into_owned(), variable)),
                    (Err(err), _) => Err(err.into()),
                    (_, Err(err)) => Err(err),
                },
            )
            .transpose()?;

        let splitter_node = node.child_by_field_name("spliter");
        let splitter = splitter_node
            .map(
                |n| match (n.text(), VariableCompiler::from_node(&n, context)) {
                    (Ok(name), Ok(variable)) => Ok((name.into_owned(), variable)),
                    (Err(err), _) => Err(err.into()),
                    (_, Err(err)) => Err(err),
                },
            )
            .transpose()?;

        let mut args= if let Some((_, variable)) = variable {
            vec![Some(Pattern::Variable(variable))]
        } else {
            bail!("stripQuote () requires a variable");
        };
        if let Some((_, spliter)) = splitter {
            args.push(Some(Pattern::Variable(spliter)));
        } else {
            bail!("stripteQuote() require a variable and a splitter variable")
        }

        let fn_index = context
            .compilation
            .built_ins
            .get_built_ins()
            .iter()
            .position(|built_in| built_in.name == "stripQuote")
            .expect("built-in stripQuote function not found");

        Ok(CallBuiltIn::new(fn_index, "stripQuote", args))
    }
}
