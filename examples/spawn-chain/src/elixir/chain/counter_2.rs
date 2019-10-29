mod label_1;
mod label_2;
mod label_3;

use std::convert::TryInto;
use std::sync::Arc;

use liblumen_alloc::erts::exception::Alloc;
use liblumen_alloc::erts::process::code;
use liblumen_alloc::erts::process::code::stack::frame::{Frame, Placement};
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;
use liblumen_alloc::ModuleFunctionArity;

pub fn place_frame_with_arguments(
    process: &Process,
    placement: Placement,
    next_pid: Term,
    output: Term,
) -> Result<(), Alloc> {
    assert!(next_pid.is_pid());
    assert!(output.is_function());
    process.stack_push(output)?;
    process.stack_push(next_pid)?;
    process.place_frame(frame(), placement);

    Ok(())
}

// Private

/// ```elixir
/// def counter(next_pid, output) when is_function(output, 1) do
///   output.("spawned")
///
///   receive do
///     n ->
///       output.("received #{n}")
///       sent = send(next_pid, n + 1)
///       output.("sent #{sent} to #{next_pid}")
///   end
/// end
/// ```
fn code(arc_process: &Arc<Process>) -> code::Result {
    arc_process.reduce();

    let next_pid = arc_process.stack_pop().unwrap();
    assert!(next_pid.is_pid());
    let output = arc_process.stack_pop().unwrap();
    // is_function(output, ...)
    let output_closure: Boxed<Closure> = output.try_into().unwrap();

    // is_function(output, 1)
    let output_module_function_arity = output_closure.module_function_arity();
    assert_eq!(output_module_function_arity.arity, 1);

    // # label 1
    // # pushed to stack: (next_pid, output)
    // # returned from called: :ok
    // # full stack: (:ok, next_pid, output)
    // receive do
    //   n ->
    //     output.("received #{n}")
    //     sent = send(next_pid, n + 1)
    //     output.("sent #{sent} to #{next_pid}")
    // end
    label_1::place_frame_with_arguments(arc_process, Placement::Replace, next_pid, output)?;

    // ```elixir
    // output.("spawned")
    // ```
    let output_data = arc_process.binary_from_str("spawned")?;
    output_closure.place_frame_with_arguments(arc_process, Placement::Push, vec![output_data])?;

    Process::call_code(arc_process)
}

fn frame() -> Frame {
    Frame::new(module_function_arity(), code)
}

fn function() -> Atom {
    Atom::try_from_str("counter").unwrap()
}

fn module_function_arity() -> Arc<ModuleFunctionArity> {
    Arc::new(ModuleFunctionArity {
        module: super::module(),
        function: function(),
        arity: 2,
    })
}
