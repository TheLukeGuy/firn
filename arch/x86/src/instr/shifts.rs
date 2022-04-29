use firn_arch_x86_macros::shift_instr;

// See ../arith.rs for all of the operation functions that are used when shift_instr! is expanded

shift_instr!(ROL);
shift_instr!(ROR);
shift_instr!(RCL);
shift_instr!(RCR);
shift_instr!(SHL);
shift_instr!(SHR);
shift_instr!(SAR);
