use inkwell::context::Context;
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn generate_code(symbols: Vec<String>) -> *mut i8 {
    let context = Context::create();
    let module = context.create_module("odin");
    let builder = context.create_builder();

    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, None);
    let entry = context.append_basic_block(function, "entry");
    builder.position_at_end(entry);
    builder.build_return(None);

    let llvm_code = module.print_to_string();
    let c_str_code = CString::new(llvm_code.to_string()).unwrap();
    
    c_str_code.into_raw()
}