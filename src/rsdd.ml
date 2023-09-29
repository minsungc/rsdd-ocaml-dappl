(* Generated by ocaml-rs *)

open! Bigarray

(* file: lib.rs *)

type rsdd_bdd_ptr
type rsdd_bdd_builder
type rsdd_cnf
type rsdd_partial_model
type rsdd_var_label
type rsdd_wmc_params_r
type rsdd_expected_utility
type rsdd_wmc_params_e_u
external mk_bdd_builder_default_order: int64 -> rsdd_bdd_builder = "mk_bdd_builder_default_order"
external bdd_new_var: rsdd_bdd_builder -> bool -> (int64 * rsdd_bdd_ptr) = "bdd_new_var"
external bdd_ite: rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr = "bdd_ite"
external bdd_and: rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr = "bdd_and"
external bdd_or: rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr = "bdd_or"
external bdd_negate: rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr = "bdd_negate"
external bdd_exactlyone: rsdd_bdd_builder -> int64 list -> rsdd_bdd_ptr = "bdd_exactlyone"
external bdd_true: rsdd_bdd_builder -> rsdd_bdd_ptr = "bdd_true"
external bdd_false: rsdd_bdd_builder -> rsdd_bdd_ptr = "bdd_false"
external bdd_is_true: rsdd_bdd_ptr -> bool = "bdd_is_true"
external bdd_is_false: rsdd_bdd_ptr -> bool = "bdd_is_false"
external bdd_is_const: rsdd_bdd_ptr -> bool = "bdd_is_const"
external bdd_eq: rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> bool = "bdd_eq"
external bdd_topvar: rsdd_bdd_ptr -> int64 = "bdd_topvar"
external bdd_low: rsdd_bdd_ptr -> rsdd_bdd_ptr = "bdd_low"
external bdd_high: rsdd_bdd_ptr -> rsdd_bdd_ptr = "bdd_high"
external bdd_wmc: rsdd_bdd_ptr -> rsdd_wmc_params_r -> float = "bdd_wmc"
external new_wmc_params_r: (float * float) list -> rsdd_wmc_params_r = "new_wmc_params_r"
external extract: rsdd_expected_utility -> float * float = "extract"
external bdd_meu: rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_var_label list -> int64 -> rsdd_wmc_params_e_u -> rsdd_expected_utility * rsdd_partial_model = "bdd_meu"
external new_wmc_params_eu: ((float * float) * (float * float)) list -> rsdd_wmc_params_e_u = "new_wmc_params_eu"
external cnf_from_dimacs: string -> rsdd_cnf = "cnf_from_dimacs"
external bdd_builder_compile_cnf: rsdd_bdd_builder -> rsdd_cnf -> rsdd_bdd_ptr = "bdd_builder_compile_cnf"
external bdd_model_count: rsdd_bdd_builder -> rsdd_bdd_ptr -> int64 = "bdd_model_count"
