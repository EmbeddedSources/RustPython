use crate::function::PyFuncArgs;
use crate::pyobject::{PyContext, PyResult, TypeProtocol};
use crate::vm::VirtualMachine;

pub fn init(context: &PyContext) {
    let ellipsis_type = context.ellipsis_type.as_object();
    extend_class!(context, ellipsis_type, {
        "__new__" => context.new_rustfunc(ellipsis_new),
        "__repr__" => context.new_rustfunc(ellipsis_repr)
    });
}

fn ellipsis_new(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(_cls, None)]);
    Ok(vm.ctx.ellipsis())
}

fn ellipsis_repr(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(_cls, None)]);
    Ok(vm.new_str("Ellipsis".to_string()))
}
