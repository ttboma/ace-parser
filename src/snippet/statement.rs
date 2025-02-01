    // let derivation = alt((
    //     Statement::cpu,
    //     Statement::config,
    //     Statement::acr,
    //     Statement::acm,
    //     Statement::acp,
    //     Statement::csim,
    //     Statement::csim_header,
    //     Statement::instruction,
    //     Statement::vector_instruction,
    //     Statement::include,
    //     Statement::acl_miscellaneous_setting,
    //     Statement::out_of_line_test_pattern,
    //     Statement::test_sequence,
    //     Statement::testbench_sequence,
    //     Statement::insn_group,
    //     Statement::sync,
    //     Statement::status,
    //     Statement::resource,
    // ));
pub const CPU: &str = "cpu {\n    ${1}\n};\n";
pub const CONFIG: &str = "config {\n    ${1}\n};\n";
pub const ACR: &str = "reg ${1} {\n    ${2}\n};\n";
pub const ACM: &str = "${1|rom,ram|} ${2} {\n    ${3}\n};\n";