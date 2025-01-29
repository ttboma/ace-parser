//! - Warning: the order of each list of keywords is not important. code must not
//!   depends on the order of the keywords. For example, if user needs keyword
//!   "frequency", user should use `optimization_policy::option::FREQUENCY`
//!   instead of `optimization_policy::OPTIONS[0]`.

/// List of all **ACE** statement pragmas
pub mod statement;

/// List of all **ACE** attribute pragmas
pub mod attribute;

/// List of all keywords that is use in an **ACE** attribute.
pub mod expression;

/// List of symbols
pub mod symbol;
