use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LabelCompletion {
    None,
    Statement,
    Attribute,
}

impl Default for LabelCompletion {
    fn default() -> Self {
        LabelCompletion::None
    }
}

impl LabelCompletion {
    pub fn completion(&self) -> Vec<&'static str> {
        match self {
            LabelCompletion::None => vec![],
            LabelCompletion::Statement => self.complete_statement(),
            LabelCompletion::Attribute => self.complete_attribute(),
        }
    }

    fn complete_statement(&self) -> Vec<&'static str> {
        vec![
            snippet::statement::CPU,
            snippet::statement::CONFIG,
            snippet::statement::ACR,
            snippet::statement::ACM,
            literal::statement::ACP,
            literal::statement::CSIM,
            literal::statement::CSIM_HEADER,
            literal::statement::INSTRUCTION_ALIAS[0],
            literal::statement::INSTRUCTION_ALIAS[1],
            literal::statement::BACKGROUND_INSTRUCTION_ALIAS[0],
            literal::statement::BACKGROUND_INSTRUCTION_ALIAS[1],
            literal::statement::VECTOR_ALIAS[0],
            literal::statement::VECTOR_ALIAS[1],
            literal::statement::UTILITY_INSTRUCTION_ALIAS[0],
            literal::statement::UTILITY_INSTRUCTION_ALIAS[1],
            literal::statement::RVV_INSTRUCTION_ALIAS[0],
            literal::statement::RVV_INSTRUCTION_ALIAS[1],
            literal::statement::ACL_MISCELLANEOUS_SETTING,
            literal::statement::TEST_PATTERN,
            literal::statement::TEST_SEQUENCE_ALIAS[0],
            literal::statement::TEST_SEQUENCE_ALIAS[1],
            literal::statement::RESOURCE,
            literal::statement::TESTBENCH_SEQUENCE_ALIAS[0],
            literal::statement::TESTBENCH_SEQUENCE_ALIAS[1],
            literal::statement::TESTBENCH_SEQUENCE_ALIAS[2],
            literal::statement::INSN_GROUP,
            literal::statement::SYNC,
            literal::statement::STATUS,
        ]
    }

    fn complete_attribute(&self) -> Vec<&'static str> {
        vec![
            literal::attribute::NAME,
            literal::attribute::VLEN,
            literal::attribute::DLEN,
            literal::attribute::ELEN,
            literal::attribute::FLEN,
            literal::attribute::FELEN,
            literal::attribute::STREAMING_PORT_WIDTH,
            literal::attribute::ADDRESS_BITS,
            literal::attribute::ENDIAN,
            literal::attribute::OPTIMIZATION_POLICY_ALIASES[0],
            literal::attribute::OPTIMIZATION_POLICY_ALIASES[1],
            literal::attribute::LM_LATENCY,
            literal::attribute::BUS_LATENCY,
            literal::attribute::EXPORT_LEVEL_AHB,
            literal::attribute::EXPORT_LEVEL_AXI,
            literal::attribute::EXPORT_LEVEL_SRAM,
            literal::attribute::EXPORT_LEVEL_PORT,
            literal::attribute::EXPORT_LEVEL_STREAMING_PORT,
            literal::attribute::RF_BUFFER_ALIASES[0],
            literal::attribute::RF_BUFFER_ALIASES[1],
            literal::attribute::GPR_BUFFER,
            literal::attribute::CUSTOM_ERROR_BITS,
            literal::attribute::CUSTOM_ERROR_EN,
            literal::attribute::INSN_QUEUE,
            literal::attribute::GROUP_IN_BUFFER,
            literal::attribute::GROUP_OUT_BUFFER,
            literal::attribute::TIMEOUT_CYCLE,
            literal::attribute::ERROR_PC,
            literal::attribute::INSN_ENCODE,
            literal::attribute::CLOCK_DOMAIN_CROSSING_STAGE,
            literal::attribute::RVV_CUSTOM_KILL,
            literal::attribute::WIDTH,
            literal::attribute::NUMBER_ALIASES[0],
            literal::attribute::NUMBER_ALIASES[1],
            literal::attribute::UTILITY_ALIASES[0],
            literal::attribute::UTILITY_ALIASES[1],
            literal::attribute::RESET,
            literal::attribute::RESET_DEFAULT,
            literal::attribute::PRIVILEGE_ALIASES[0],
            literal::attribute::PRIVILEGE_ALIASES[1],
            literal::attribute::ACCESS_TYPE,
            literal::attribute::LLVM_RA,
            literal::attribute::INTERFACE,
            literal::attribute::LATENCY,
            literal::attribute::CONTENT,
            literal::attribute::CONTENT_DEFAULT,
            literal::attribute::ERROR_DETECT,
            literal::attribute::BYTE_ENABLE_ALIASES[0],
            literal::attribute::BYTE_ENABLE_ALIASES[1],
            literal::attribute::WRITE_STROBE,
            literal::attribute::MAX_BURST_LENGTH,
            literal::attribute::EXPORT_LEVEL,
            literal::attribute::IO_TYPE,
            literal::attribute::BUFFER_ALIASES[0],
            literal::attribute::BUFFER_ALIASES[1],
            literal::attribute::CSIM,
            literal::attribute::CHISEL,
            literal::attribute::SPINALHDL,
            literal::attribute::MISC_SETTING,
            literal::attribute::OPERAND_ALIASES[0],
            literal::attribute::OPERAND_ALIASES[1],
            literal::attribute::IMPLIED_OPERAND_ALIASES[0],
            literal::attribute::IMPLIED_OPERAND_ALIASES[1],
            literal::attribute::CSR_OPERAND_ALIASES[0],
            literal::attribute::CSR_OPERAND_ALIASES[1],
            literal::attribute::EXTRA_DECODING_TYPE_SIGNAL,
            literal::attribute::STREAMING_PORT,
            literal::attribute::SIDE_EFFECT,
            literal::attribute::CSIM_CYCLE,
            literal::attribute::CHISEL_CYCLE,
            literal::attribute::SPINALHDL_CYCLE,
            literal::attribute::BLOCKING,
            literal::attribute::INTERRUPT,
            literal::attribute::OUTSTANDING_INSN_NUM,
            literal::attribute::UTILITY_KIND,
            literal::attribute::TEST_PATTERN,
            literal::attribute::THROUGHPUT,
            literal::attribute::CYCLE_PER_RESULT_ALIASES[0],
            literal::attribute::CYCLE_PER_RESULT_ALIASES[1],
            literal::attribute::VECTOR_MASK,
            literal::attribute::VECTOR_UNIT,
            literal::attribute::CUSTOM_KILL,
            literal::attribute::INSTANTIATE,
            literal::attribute::TEST_SEQUENCE_INIT[0],
            literal::attribute::TEST_SEQUENCE_INIT[1],
            literal::attribute::SEQUENCE_ALIASES[0],
            literal::attribute::SEQUENCE_ALIASES[1],
            literal::attribute::LIST,
            literal::attribute::LATENCY_OVERHEAD,
            literal::attribute::CSIM_INITIALIZATION_ALIASES[0],
            literal::attribute::CSIM_INITIALIZATION_ALIASES[1],
            literal::attribute::CHISEL_INITIALIZATION_ALIASES[0],
            literal::attribute::CHISEL_INITIALIZATION_ALIASES[1],
            literal::attribute::SPINALHDL_INITIALIZATION_ALIASES[0],
            literal::attribute::SPINALHDL_INITIALIZATION_ALIASES[1],
            literal::attribute::LOOP_TYPE,
            literal::attribute::STRIDE,
            literal::attribute::BASE_OPCODE,
            literal::attribute::MARCH,
            literal::attribute::FRF_BUFFER,
        ]
    }
}
