// This allows to compile the examples

#[path = "expand/as_expression.rs"]
mod as_expression;
#[path = "expand/as_expression.expanded.rs"]
mod as_expression_expanded;

#[path = "expand/flat.rs"]
mod flat;
#[path = "expand/flat.expanded.rs"]
mod flat_expanded;

#[path = "expand/misc.rs"]
mod misc;
#[path = "expand/misc.expanded.rs"]
mod misc_expanded;

#[path = "expand/nested4.rs"]
mod nested4;
#[path = "expand/nested4.expanded.rs"]
mod nested4_expanded;

#[path = "expand/support_empty_braces.rs"]
mod support_empty_braces;
#[path = "expand/support_empty_braces.expanded.rs"]
mod support_empty_braces_expanded;
