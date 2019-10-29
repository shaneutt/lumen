use std::sync::Arc;

use liblumen_alloc::atom;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::exception::Alloc;
use liblumen_alloc::erts::process::code::stack::frame::{Frame, Placement};
use liblumen_alloc::erts::process::code::{self, result_from_exception};
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::erts::ModuleFunctionArity;

use crate::element;

/// ```elixir
/// case Lumen.Web.Element.set_attribute(element, "data-attribute", "data-value") do
///   :ok -> ...
///   {:error, {:name, name} -> ...
/// end
/// ``
pub fn place_frame_with_arguments(
    process: &Process,
    placement: Placement,
    element: Term,
) -> Result<(), Alloc> {
    process.stack_push(element)?;
    process.place_frame(frame(), placement);

    Ok(())
}

// Private

fn code(arc_process: &Arc<Process>) -> code::Result {
    arc_process.reduce();

    let element = arc_process.stack_pop().unwrap();

    match native(element) {
        Ok(ok) => {
            arc_process.return_from_call(ok)?;

            Process::call_code(arc_process)
        }
        Err(exception) => result_from_exception(arc_process, exception),
    }
}

fn frame() -> Frame {
    Frame::new(module_function_arity(), code)
}

fn function() -> Atom {
    Atom::try_from_str("set_attribute").unwrap()
}

fn module_function_arity() -> Arc<ModuleFunctionArity> {
    Arc::new(ModuleFunctionArity {
        module: super::module(),
        function: function(),
        arity: 1,
    })
}

fn native(element_term: Term) -> exception::Result<Term> {
    let element = element::from_term(element_term)?;

    element.remove();

    Ok(atom!("ok"))
}
