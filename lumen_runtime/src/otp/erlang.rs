//! Mirrors [erlang](http://erlang::org/doc/man/erlang::html) module

pub mod abs_1;
pub mod add_2;
pub mod and_2;
pub mod andalso_2;
pub mod append_element_2;
pub mod apply_3;
pub mod are_equal_after_conversion_2;
pub mod are_exactly_equal_2;
pub mod are_exactly_not_equal_2;
pub mod are_not_equal_after_conversion_2;
pub mod atom_to_binary_2;
pub mod atom_to_list_1;
pub mod band_2;
pub mod binary_part_2;
pub mod binary_part_3;
pub mod binary_to_atom_2;
pub mod binary_to_existing_atom_2;
pub mod binary_to_float_1;
pub mod binary_to_integer_1;
pub mod binary_to_integer_2;
pub mod binary_to_list_1;
pub mod binary_to_list_3;
pub mod binary_to_term_1;
pub mod binary_to_term_2;
pub mod bit_size_1;
pub mod bitstring_to_list_1;
pub mod bnot_1;
pub mod bor_2;
pub mod bsl_2;
pub mod bsr_2;
pub mod bxor_2;
pub mod byte_size_1;
pub mod cancel_timer_1;
pub mod cancel_timer_2;
pub mod ceil_1;
pub mod concatenate_2;
pub mod convert_time_unit_3;
pub mod delete_element_2;
pub mod demonitor_2;
pub mod div_2;
pub mod divide_2;
pub mod element_2;
pub mod error_1;
pub mod error_2;
pub mod exit_1;
pub mod hd_1;
pub mod insert_element_3;
pub mod is_alive_0;
pub mod is_atom_1;
pub mod is_binary_1;
pub mod is_bitstring_1;
pub mod is_boolean_1;
pub mod is_equal_or_less_than_2;
pub mod is_float_1;
pub mod is_function_1;
pub mod is_function_2;
pub mod is_greater_than_2;
pub mod is_greater_than_or_equal_2;
pub mod is_integer_1;
pub mod is_less_than_2;
pub mod is_list_1;
pub mod is_map_1;
pub mod is_map_key_2;
pub mod is_number_1;
pub mod is_pid_1;
pub mod is_record_2;
pub mod is_record_3;
pub mod is_reference_1;
pub mod is_tuple_1;
pub mod length_1;
pub mod link_1;
pub mod list_to_atom_1;
pub mod list_to_binary_1;
pub mod list_to_bitstring_1;
pub mod list_to_existing_atom_1;
pub mod list_to_pid_1;
pub mod list_to_tuple_1;
pub mod make_ref_0;
pub mod map_get_2;
pub mod map_size_1;
pub mod max_2;
pub mod min_2;
pub mod monitor_2;
pub mod monotonic_time_0;
pub mod monotonic_time_1;
pub mod multiply_2;
pub mod negate_1;
pub mod node_0;
pub mod not_1;
pub mod number_or_badarith_1;
pub mod or_2;
pub mod orelse_2;
pub mod process_flag_2;
pub mod process_info_2;
pub mod raise_3;
pub mod read_timer_1;
pub mod read_timer_2;
pub mod register_2;
pub mod registered_0;
pub mod rem_2;
pub mod self_0;
pub mod send_2;
pub mod send_3;
pub mod send_after_3;
pub mod send_after_4;
pub mod setelement_3;
pub mod size_1;
pub mod spawn_3;
pub mod spawn_apply_3;
pub mod spawn_link_3;
pub mod spawn_opt_4;
pub mod split_binary_2;
pub mod start_timer_3;
pub mod start_timer_4;
pub mod subtract_2;
pub mod unlink_1;

// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod tests;

use core::convert::TryInto;

use alloc::sync::Arc;

use liblumen_alloc::erts::exception::{Exception, Result};
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::{
    atom_unchecked, AsTerm, Atom, Boxed, Cons, ImproperList, Term, Tuple, TypedTerm,
};
use liblumen_alloc::{badarg, throw};

use crate::registry::{self, pid_to_self_or_process};
use crate::time::monotonic::{self, Milliseconds};
use crate::timer::start::ReferenceFrame;
use crate::timer::{self, Timeout};
use liblumen_alloc::erts::process::alloc::heap_alloc::HeapAlloc;

pub const MAX_SHIFT: usize = std::mem::size_of::<isize>() * 8 - 1;

pub fn module() -> Atom {
    Atom::try_from_str("erlang").unwrap()
}

pub fn subtract_list_2(minuend: Term, subtrahend: Term, process: &Process) -> Result {
    match (
        minuend.to_typed_term().unwrap(),
        subtrahend.to_typed_term().unwrap(),
    ) {
        (TypedTerm::Nil, TypedTerm::Nil) => Ok(minuend),
        (TypedTerm::Nil, TypedTerm::List(subtrahend_cons)) => {
            if subtrahend_cons.is_proper() {
                Ok(minuend)
            } else {
                Err(badarg!().into())
            }
        }
        (TypedTerm::List(minuend_cons), TypedTerm::Nil) => {
            if minuend_cons.is_proper() {
                Ok(minuend)
            } else {
                Err(badarg!().into())
            }
        }
        (TypedTerm::List(minuend_cons), TypedTerm::List(subtrahend_cons)) => {
            match minuend_cons
                .into_iter()
                .collect::<std::result::Result<Vec<Term>, _>>()
            {
                Ok(mut minuend_vec) => {
                    for result in subtrahend_cons.into_iter() {
                        match result {
                            Ok(subtrahend_element) => minuend_vec.remove_item(&subtrahend_element),
                            Err(ImproperList { .. }) => return Err(badarg!().into()),
                        };
                    }

                    process
                        .list_from_slice(&minuend_vec)
                        .map_err(|error| error.into())
                }
                Err(ImproperList { .. }) => Err(badarg!().into()),
            }
        }
        _ => Err(badarg!().into()),
    }
}

pub fn throw_1(reason: Term) -> Result {
    Err(throw!(reason).into())
}

pub fn tl_1(list: Term) -> Result {
    let cons: Boxed<Cons> = list.try_into()?;

    Ok(cons.tail)
}

pub fn tuple_size_1(tuple: Term, process: &Process) -> Result {
    let tuple: Boxed<Tuple> = tuple.try_into()?;
    let size = process.integer(tuple.len())?;

    Ok(size)
}

pub fn tuple_to_list_1(tuple: Term, process: &Process) -> Result {
    let tuple: Boxed<Tuple> = tuple.try_into()?;
    let mut heap = process.acquire_heap();
    let mut acc = Term::NIL;

    for element in tuple.iter().rev() {
        acc = heap.cons(element, acc)?;
    }

    Ok(acc)
}

pub fn unregister_1(name: Term) -> Result {
    let atom: Atom = name.try_into()?;

    if registry::unregister(&atom) {
        Ok(true.into())
    } else {
        Err(badarg!().into())
    }
}

pub fn whereis_1(name: Term) -> Result {
    let atom: Atom = name.try_into()?;

    let option = registry::atom_to_process(&atom).map(|arc_process| arc_process.pid());

    let term = match option {
        Some(pid) => unsafe { pid.as_term() },
        None => atom_unchecked("undefined"),
    };

    Ok(term)
}

/// `xor/2` infix operator.
///
/// **NOTE: NOT SHORT-CIRCUITING!**
pub fn xor_2(left_boolean: Term, right_boolean: Term) -> Result {
    boolean_infix_operator!(left_boolean, right_boolean, ^)
}

// Private

fn cancel_timer(
    timer_reference: Term,
    options: timer::cancel::Options,
    process: &Process,
) -> Result {
    match timer_reference.to_typed_term().unwrap() {
        TypedTerm::Boxed(unboxed_timer_reference) => {
            match unboxed_timer_reference.to_typed_term().unwrap() {
                TypedTerm::Reference(ref reference) => {
                    let canceled = timer::cancel(reference);

                    let term = if options.info {
                        let mut heap = process.acquire_heap();
                        let canceled_term = match canceled {
                            Some(milliseconds_remaining) => heap.integer(milliseconds_remaining)?,
                            None => false.into(),
                        };

                        if options.r#async {
                            let cancel_timer_message = heap.tuple_from_slice(&[
                                atom_unchecked("cancel_timer"),
                                timer_reference,
                                canceled_term,
                            ])?;
                            process.send_from_self(cancel_timer_message);

                            atom_unchecked("ok")
                        } else {
                            canceled_term
                        }
                    } else {
                        atom_unchecked("ok")
                    };

                    Ok(term)
                }
                _ => Err(badarg!().into()),
            }
        }
        _ => Err(badarg!().into()),
    }
}

fn is_record(term: Term, record_tag: Term, size: Option<Term>) -> Result {
    match term.to_typed_term().unwrap() {
        TypedTerm::Boxed(boxed) => match boxed.to_typed_term().unwrap() {
            TypedTerm::Tuple(tuple) => {
                match record_tag.to_typed_term().unwrap() {
                    TypedTerm::Atom(_) => {
                        let len = tuple.len();

                        let tagged = if 0 < len {
                            let element = tuple[0];

                            match size {
                                Some(size_term) => {
                                    let size_usize: usize = size_term.try_into()?;

                                    (element == record_tag) & (len == size_usize)
                                }
                                None => element == record_tag,
                            }
                        } else {
                            // even if the `record_tag` cannot be checked, the `size` is still type
                            // checked
                            if let Some(size_term) = size {
                                let _: usize = size_term.try_into()?;
                            }

                            false
                        };

                        Ok(tagged.into())
                    }
                    _ => Err(badarg!().into()),
                }
            }
            _ => Ok(false.into()),
        },
        _ => Ok(false.into()),
    }
}

fn list_to_string(list: Term) -> std::result::Result<String, Exception> {
    match list.to_typed_term().unwrap() {
        TypedTerm::Nil => Ok("".to_owned()),
        TypedTerm::List(cons) => cons
            .into_iter()
            .map(|result| match result {
                Ok(term) => {
                    let c: char = term.try_into()?;

                    Ok(c)
                }
                Err(ImproperList { .. }) => Err(badarg!().into()),
            })
            .collect::<std::result::Result<String, Exception>>(),
        _ => Err(badarg!().into()),
    }
}

fn read_timer(timer_reference: Term, options: timer::read::Options, process: &Process) -> Result {
    match timer_reference.to_typed_term().unwrap() {
        TypedTerm::Boxed(unboxed_timer_reference) => {
            match unboxed_timer_reference.to_typed_term().unwrap() {
                TypedTerm::Reference(ref local_reference) => {
                    let read = timer::read(local_reference);
                    let mut heap = process.acquire_heap();

                    let read_term = match read {
                        Some(milliseconds_remaining) => heap.integer(milliseconds_remaining)?,
                        None => false.into(),
                    };

                    let term = if options.r#async {
                        let read_timer_message = heap.tuple_from_slice(&[
                            atom_unchecked("read_timer"),
                            timer_reference,
                            read_term,
                        ])?;
                        process.send_from_self(read_timer_message);

                        atom_unchecked("ok")
                    } else {
                        read_term
                    };

                    Ok(term)
                }
                _ => Err(badarg!().into()),
            }
        }
        _ => Err(badarg!().into()),
    }
}

fn start_timer(
    time: Term,
    destination: Term,
    timeout: Timeout,
    message: Term,
    options: timer::start::Options,
    arc_process: Arc<Process>,
) -> Result {
    if time.is_integer() {
        let reference_frame_milliseconds: Milliseconds = time.try_into()?;

        let absolute_milliseconds = match options.reference_frame {
            ReferenceFrame::Relative => {
                monotonic::time_in_milliseconds() + reference_frame_milliseconds
            }
            ReferenceFrame::Absolute => reference_frame_milliseconds,
        };

        match destination.to_typed_term().unwrap() {
            // Registered names are looked up at time of send
            TypedTerm::Atom(destination_atom) => timer::start(
                absolute_milliseconds,
                timer::Destination::Name(destination_atom),
                timeout,
                message,
                &arc_process,
            )
            .map_err(|error| error.into()),
            // PIDs are looked up at time of create.  If they don't exist, they still return a
            // LocalReference.
            TypedTerm::Pid(destination_pid) => {
                match pid_to_self_or_process(destination_pid, &arc_process) {
                    Some(pid_arc_process) => timer::start(
                        absolute_milliseconds,
                        timer::Destination::Process(Arc::downgrade(&pid_arc_process)),
                        timeout,
                        message,
                        &arc_process,
                    )
                    .map_err(|error| error.into()),
                    None => make_ref_0::native(&arc_process),
                }
            }
            _ => Err(badarg!().into()),
        }
    } else {
        Err(badarg!().into())
    }
}
