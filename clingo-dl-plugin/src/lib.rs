extern crate clingo;
extern crate clingo_dl_sys;
extern crate clingo_sys;
use clingo::{
    ast,
    theory::{Theory, TheoryValue},
    ControlCtx, GenericControl, Id, Model, Options, Statistics, Symbol,
};
use clingo_sys::clingo_control;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct DLTheory {
    theory: NonNull<clingo_dl_sys::clingodl_theory>,
}
impl<'a> DLTheory {
    /// creates the theory
    pub fn create() -> DLTheory {
        let mut theory_ptr = std::ptr::null_mut();
        unsafe { clingo_dl_sys::clingodl_create(&mut theory_ptr) };
        match NonNull::new(theory_ptr) {
            Some(theory) => DLTheory { theory },
            None => panic!("Tried creating NonNull from a null pointer."),
        }
    }
}
impl Drop for DLTheory {
    fn drop(&mut self) {
        let success = unsafe { clingo_dl_sys::clingodl_destroy(self.theory.as_ptr()) };
        if !success {
            panic!("call clingodl_destroy returned false")
        }
    }
}
/// An iterator over dl theory values.
pub struct DLTheoryAssignment<'a> {
    dl_theory: &'a DLTheory,
    thread_id: Id,
    index: usize,
}
impl<'a> Iterator for DLTheoryAssignment<'a> {
    type Item = (Symbol, TheoryValue);

    fn next(&mut self) -> Option<(Symbol, TheoryValue)> {
        if !unsafe {
            clingo_dl_sys::clingodl_assignment_next(
                self.dl_theory.theory.as_ptr(),
                self.thread_id.into(),
                &mut self.index,
            )
        } {
            None
        } else if unsafe {
            clingo_dl_sys::clingodl_assignment_has_value(
                self.dl_theory.theory.as_ptr(),
                self.thread_id.into(),
                self.index,
            )
        } {
            let sym: clingo_sys::clingo_symbol_t = unsafe {
                clingo_dl_sys::clingodl_get_symbol(self.dl_theory.theory.as_ptr(), self.index)
            };
            let sym = sym.into();
            let value_internal = clingo_dl_sys::clingodl_value__bindgen_ty_1 { int_number: 0 };
            let mut value = clingo_dl_sys::clingodl_value {
                type_: 0,
                __bindgen_anon_1: value_internal,
            };
            unsafe {
                clingo_dl_sys::clingodl_assignment_get_value(
                    self.dl_theory.theory.as_ptr(),
                    self.thread_id.into(),
                    self.index,
                    &mut value,
                )
            };
            match value.type_ {
                0 => Some((
                    sym,
                    TheoryValue::IntNumber(unsafe { value.__bindgen_anon_1.int_number } as u64),
                )),
                1 => Some((
                    sym,
                    TheoryValue::DoubleNumber(unsafe { value.__bindgen_anon_1.double_number }),
                )),
                2 => {
                    let value = unsafe { value.__bindgen_anon_1.symbol };
                    Some((sym, TheoryValue::Symbol(value.into())))
                }
                x => panic!("unexpected DLTheoryValue {}", x),
            }
        } else {
            None
        }
    }
}
impl<'a> Theory<'a> for DLTheory {
    fn assignment(&'a self, thread_id: Id) -> Box<dyn Iterator<Item = (Symbol, TheoryValue)> + 'a> {
        let mut index = 0;
        unsafe {
            clingo_dl_sys::clingodl_assignment_begin(
                self.theory.as_ptr(),
                thread_id.into(),
                &mut index,
            )
        }
        Box::new(DLTheoryAssignment {
            dl_theory: self,
            thread_id,
            index,
        })
    }
    /// registers the theory with the control
    fn register<Ctx>(&mut self, ctl: &mut GenericControl<Ctx>) -> bool
    where
        Ctx: ControlCtx,
    {
        let nn: NonNull<clingo_control> = ctl.into();
        unsafe { clingo_dl_sys::clingodl_register(self.theory.as_ptr(), nn.as_ptr()) }
    }
    /// Rewrite statements before adding them via the given callback.
    // fn rewrite_statement(
    //     &mut self,
    //     stmt: &ast::Statement,
    //     builder: &mut ast::ProgramBuilder,
    // ) -> bool {
    //     let add = unsafe_program_builder_add;
    //     let nn: NonNull<clingo_ast> = stmt.into();
    //     let pb: *mut clingo_sys::clingo_program_builder = builder.into();
    //     unsafe {
    //         clingo_dl_sys::clingodl_rewrite_ast(
    //             self.theory.as_ptr(),
    //             nn.as_ptr(),
    //             Some(add),
    //             pb as *mut ::std::os::raw::c_void,
    //         )
    //     }
    // }
    /// adds head/body marker to `&diff` theory atoms
    fn rewrite_statement(
        &mut self,
        stm: &ast::Statement,
        builder: &mut ast::ProgramBuilder,
    ) -> bool {
        let stm_clone = stm.clone();
        match stm_clone.is_a().unwrap() {
            ast::StatementIsA::Rule(rule) => {
                let loc = rule.location();
                let body = rule.body();
                let mut new_body = vec![];
                for bl in body {
                    new_body.push(rewrite_body_literal(bl));
                }
                // initialize the rule
                let head = rule.head();
                let new_head = rewrite_head(head);
                let rule = ast::rule(&loc, new_head, &new_body).unwrap();
                eprintln!("new rule {}", rule);

                // add the rewritten rule to the program builder
                builder
                    .add(&rule.into())
                    .expect("Failed to add Rule to ProgramBuilder.");
            }
            _ => {
                // pass through all statements that are not rules
                builder
                    .add(&stm)
                    .expect("Failed to add Statement to ProgramBuilder.");
            }
        }
        true
    }

    /// prepare the theory between grounding and solving
    fn prepare<Ctx>(&mut self, ctl: &mut GenericControl<Ctx>) -> bool
    where
        Ctx: ControlCtx,
    {
        let nn: NonNull<clingo_control> = ctl.into();
        unsafe { clingo_dl_sys::clingodl_prepare(self.theory.as_ptr(), nn.as_ptr()) }
    }
    /// add options for your theory
    fn register_options(&mut self, options: &mut Options) -> bool {
        unsafe { clingo_dl_sys::clingodl_register_options(self.theory.as_ptr(), options.into()) }
    }
    /// validate options for your theory
    fn validate_options(&mut self) -> bool {
        unsafe { clingo_dl_sys::clingodl_validate_options(self.theory.as_ptr()) }
    }
    /// callback on every model
    fn on_model(&mut self, model: &mut Model) -> bool {
        unsafe { clingo_dl_sys::clingodl_on_model(self.theory.as_ptr(), model.into()) }
    }
    /// callback on statistic updates
    /// please add a subkey with the name of your theory
    fn on_statistics(&mut self, step: &mut Statistics, accu: &mut Statistics) -> bool {
        unsafe {
            clingo_dl_sys::clingodl_on_statistics(self.theory.as_ptr(), step.into(), accu.into())
        }
    }
    /// obtain a symbol index which can be used to get the value of a symbol
    /// returns true if the symbol exists
    /// does not throw
    fn lookup_symbol(&mut self, symbol: Symbol, index: &mut usize) -> bool {
        unsafe { clingo_dl_sys::clingodl_lookup_symbol(self.theory.as_ptr(), symbol.into(), index) }
    }
    /// obtain the symbol at the given index
    /// does not throw
    fn get_symbol(&mut self, index: usize) -> Symbol {
        let sym: clingo_sys::clingo_symbol_t =
            unsafe { clingo_dl_sys::clingodl_get_symbol(self.theory.as_ptr(), index) };
        sym.into()
    }
    /// configure theory manually (without using clingo's options facility)
    /// Note that the theory has to be configured before registering it and cannot be reconfigured.
    fn configure(&mut self, key: &str, value: &str) -> bool {
        unsafe {
            clingo_dl_sys::clingodl_configure(
                self.theory.as_ptr(),
                key.as_ptr() as *const i8,
                value.as_ptr() as *const i8,
            )
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Marker {
    Head,
    Body,
}
fn rewrite_head(head: ast::Head) -> ast::Head {
    match head.is_a().unwrap() {
        ast::HeadIsA::TheoryAtom(theory_atom) => {
            rewrite_theory_atom(theory_atom, Marker::Head).into()
        }

        ast::HeadIsA::Literal(literal) => literal.into(),
        ast::HeadIsA::Aggregate(aggregate) => aggregate.into(),
        ast::HeadIsA::HeadAggregate(head_aggregate) => head_aggregate.into(),
        ast::HeadIsA::Disjunction(disjunction) => disjunction.into(),
    }
}
fn rewrite_body_literal(bl: ast::BodyLiteral) -> ast::BodyLiteral {
    match bl.is_a().unwrap() {
        ast::BodyLiteralIsA::Literal(literal) => literal.into(),
        ast::BodyLiteralIsA::ConditionalLiteral(cond_literal) => cond_literal.into(),
        ast::BodyLiteralIsA::TheoryAtom(theory_atom) => {
            rewrite_theory_atom(theory_atom, Marker::Body).into()
        }
    }
}
fn rewrite_theory_atom(ta: ast::TheoryAtom, marker: Marker) -> ast::TheoryAtom {
    let mut ta_clone = ta.clone();
    let term = ta.term();
    let term_location = term.location();
    match term.is_a().unwrap() {
        ast::TermIsA::Function(function) => {
            let name = function.name();
            if name == "diff" {
                match marker {
                    Marker::Body => {
                        let new_term =
                            ast::function(&term_location, &"__diff_b", &[], false).unwrap();
                        // eprintln!("new term(function): {}", new_term.to_string().unwrap());
                        ta_clone.set_term(new_term.into());
                        // eprintln!("new TheoryAtom: {:?}", ta_clone);
                    }
                    Marker::Head => {
                        let new_term =
                            ast::function(&term_location, &"__diff_h", &[], false).unwrap();
                        // eprintln!("new term(function): {}", new_term.to_string().unwrap());
                        ta_clone.set_term(new_term.into());
                        // eprintln!("new TheoryAtom: {:?}", ta_clone);
                    }
                }
            }
        }
        x => panic!("Unexpected variant {:?}", x),
    }
    ta_clone
}

// unsafe extern "C" fn unsafe_program_builder_add(
//     statement: *const clingo_sys::clingo_ast_t,
//     data: *mut ::std::os::raw::c_void,
// ) -> bool {
//     let builder = data as *mut clingo_sys::clingo_program_builder;
//     clingo_sys::clingo_program_builder_add(builder, statement)
// }
