pub type clingodl_value_type_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clingodl_value {
    pub type_: clingodl_value_type_t,
    pub __bindgen_anon_1: clingodl_value__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union clingodl_value__bindgen_ty_1 {
    pub int_number: ::std::os::raw::c_int,
    pub double_number: f64,
    pub symbol: clingo_symbol_t,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_clingodl_value__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<clingodl_value__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(clingodl_value__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<clingodl_value__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(clingodl_value__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<clingodl_value__bindgen_ty_1>())).int_number as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingodl_value__bindgen_ty_1),
            "::",
            stringify!(int_number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<clingodl_value__bindgen_ty_1>())).double_number as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingodl_value__bindgen_ty_1),
            "::",
            stringify!(double_number)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<clingodl_value__bindgen_ty_1>())).symbol as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingodl_value__bindgen_ty_1),
            "::",
            stringify!(symbol)
        )
    );
}
#[test]
fn bindgen_test_layout_clingodl_value() {
    assert_eq!(
        ::std::mem::size_of::<clingodl_value>(),
        16usize,
        concat!("Size of: ", stringify!(clingodl_value))
    );
    assert_eq!(
        ::std::mem::align_of::<clingodl_value>(),
        8usize,
        concat!("Alignment of ", stringify!(clingodl_value))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<clingodl_value>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(clingodl_value),
            "::",
            stringify!(type_)
        )
    );
}
pub type clingodl_value_t = clingodl_value;
#[doc = "! Callback to rewrite statements (see ::clingodl_rewrite_statement)."]
pub type clingodl_rewrite_callback_t = ::std::option::Option<
    unsafe extern "C" fn(
        statement: *const clingo_ast_statement_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clingodl_theory {
    _unused: [u8; 0],
}
pub type clingodl_theory_t = clingodl_theory;
extern "C" {
    #[doc = "! creates the theory"]
    pub fn clingodl_create(theory: *mut *mut clingodl_theory_t) -> bool;
}
extern "C" {
    #[doc = "! registers the theory with the control"]
    pub fn clingodl_register(
        theory: *mut clingodl_theory_t,
        control: *mut clingo_control_t,
    ) -> bool;
}
extern "C" {
    #[doc = "! Rewrite statements before adding them via the given callback."]
    pub fn clingodl_rewrite_statement(
        theory: *mut clingodl_theory_t,
        stm: *const clingo_ast_statement_t,
        add: clingodl_rewrite_callback_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    #[doc = "! prepare the theory between grounding and solving"]
    pub fn clingodl_prepare(theory: *mut clingodl_theory_t, control: *mut clingo_control_t)
        -> bool;
}
extern "C" {
    #[doc = "! destroys the theory, currently no way to unregister a theory"]
    pub fn clingodl_destroy(theory: *mut clingodl_theory_t) -> bool;
}
extern "C" {
    #[doc = "! configure theory manually (without using clingo's options facility)"]
    #[doc = "! Note that the theory has to be configured before registering it and cannot be reconfigured."]
    pub fn clingodl_configure(
        theory: *mut clingodl_theory_t,
        key: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    #[doc = "! add options for your theory"]
    pub fn clingodl_register_options(
        theory: *mut clingodl_theory_t,
        options: *mut clingo_options_t,
    ) -> bool;
}
extern "C" {
    #[doc = "! validate options for your theory"]
    pub fn clingodl_validate_options(theory: *mut clingodl_theory_t) -> bool;
}
extern "C" {
    #[doc = "! callback on every model"]
    pub fn clingodl_on_model(theory: *mut clingodl_theory_t, model: *mut clingo_model_t) -> bool;
}
extern "C" {
    #[doc = "! obtain a symbol index which can be used to get the value of a symbol"]
    #[doc = "! returns true if the symbol exists"]
    #[doc = "! does not throw"]
    pub fn clingodl_lookup_symbol(
        theory: *mut clingodl_theory_t,
        symbol: clingo_symbol_t,
        index: *mut usize,
    ) -> bool;
}
extern "C" {
    #[doc = "! obtain the symbol at the given index"]
    #[doc = "! does not throw"]
    pub fn clingodl_get_symbol(theory: *mut clingodl_theory_t, index: usize) -> clingo_symbol_t;
}
extern "C" {
    #[doc = "! initialize index so that it can be used with clingodl_assignment_next"]
    #[doc = "! does not throw"]
    pub fn clingodl_assignment_begin(
        theory: *mut clingodl_theory_t,
        thread_id: u32,
        index: *mut usize,
    );
}
extern "C" {
    #[doc = "! move to the next index that has a value"]
    #[doc = "! returns true if the updated index is valid"]
    #[doc = "! does not throw"]
    pub fn clingodl_assignment_next(
        theory: *mut clingodl_theory_t,
        thread_id: u32,
        index: *mut usize,
    ) -> bool;
}
extern "C" {
    #[doc = "! check if the symbol at the given index has a value"]
    #[doc = "! does not throw"]
    pub fn clingodl_assignment_has_value(
        theory: *mut clingodl_theory_t,
        thread_id: u32,
        index: usize,
    ) -> bool;
}
extern "C" {
    #[doc = "! get the symbol and it's value at the given index"]
    #[doc = "! does not throw"]
    pub fn clingodl_assignment_get_value(
        theory: *mut clingodl_theory_t,
        thread_id: u32,
        index: usize,
        value: *mut clingodl_value_t,
    );
}
extern "C" {
    #[doc = "! callback on statistic updates"]
    #[doc = " please add a subkey with the name of your theory"]
    pub fn clingodl_on_statistics(
        theory: *mut clingodl_theory_t,
        step: *mut clingo_statistics_t,
        accu: *mut clingo_statistics_t,
    ) -> bool;
}
