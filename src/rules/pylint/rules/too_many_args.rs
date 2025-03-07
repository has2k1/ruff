use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::define_violation;
use crate::registry::Diagnostic;
use crate::violation::Violation;

use ruff_macros::derive_message_formats;
use rustpython_ast::{Arguments, Stmt};

define_violation!(
    pub struct TooManyArgs {
        pub c_args: usize,
        pub max_args: usize,
    }
);

impl Violation for TooManyArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyArgs { c_args, max_args } = self;
        format!("Too many arguments to function call ({c_args}/{max_args})")
    }
}

/// PLR0913
pub fn too_many_args(checker: &mut Checker, args: &Arguments, stmt: &Stmt) {
    let num_args = args
        .args
        .iter()
        .filter(|arg| !checker.settings.dummy_variable_rgx.is_match(&arg.node.arg))
        .count();
    if num_args > checker.settings.pylint.max_args {
        checker.diagnostics.push(Diagnostic::new(
            TooManyArgs {
                c_args: num_args,
                max_args: checker.settings.pylint.max_args,
            },
            Range::from_located(stmt),
        ));
    }
}
