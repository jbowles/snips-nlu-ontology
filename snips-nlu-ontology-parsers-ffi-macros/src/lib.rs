extern crate failure;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate snips_nlu_ontology;
extern crate snips_nlu_ontology_ffi_macros;
extern crate snips_nlu_ontology_parsers;
#[macro_use]
extern crate ffi_utils;

type Result<T> = ::std::result::Result<T, ::failure::Error>;

mod builtin_entity_parser;
mod gazetteer_entity_parser;

pub use builtin_entity_parser::*;
pub use gazetteer_entity_parser::*;

#[macro_export]
macro_rules! export_nlu_ontology_parsers_c_symbols {
    () => {
        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_create_builtin_entity_parser(
            ptr: *mut *const $crate::CBuiltinEntityParser,
            json_config: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::create_builtin_entity_parser(ptr, json_config))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_persist_builtin_entity_parser(
            ptr: *const $crate::CBuiltinEntityParser,
            path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::persist_builtin_entity_parser(ptr, path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_load_builtin_entity_parser(
            ptr: *mut *const $crate::CBuiltinEntityParser,
            parser_path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::load_builtin_entity_parser(ptr, parser_path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_extract_builtin_entities(
            ptr: *const $crate::CBuiltinEntityParser,
            sentence: *const ::libc::c_char,
            filter_entity_kinds: *const ::ffi_utils::CStringArray,
            results: *mut *const snips_nlu_ontology_ffi_macros::CBuiltinEntityArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::extract_builtin_entity_c(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_extract_builtin_entities_json(
            ptr: *const $crate::CBuiltinEntityParser,
            sentence: *const ::libc::c_char,
            filter_entity_kinds: *const ::ffi_utils::CStringArray,
            results: *mut *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::extract_builtin_entity_json(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_builtin_entity_array(
            ptr: *mut ::snips_nlu_ontology_ffi_macros::CBuiltinEntityArray,
        ) -> ::ffi_utils::SNIPS_RESULT {
            use ::ffi_utils::RawPointerConverter;
            use ::snips_nlu_ontology_ffi_macros::CBuiltinEntityArray;
            wrap!(unsafe { CBuiltinEntityArray::from_raw_pointer(ptr) })
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_builtin_entity_parser(
            ptr: *mut $crate::CBuiltinEntityParser,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::destroy_builtin_entity_parser(ptr))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_build_gazetteer_entity_parser(
            ptr: *mut *const $crate::CGazetteerEntityParser,
            json_config: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::build_gazetteer_entity_parser(ptr, json_config))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_load_gazetteer_entity_parser(
            ptr: *mut *const $crate::CGazetteerEntityParser,
            parser_path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::load_gazetteer_entity_parser(ptr, parser_path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_persist_gazetteer_entity_parser(
            ptr: *const $crate::CGazetteerEntityParser,
            path: *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::persist_gazetteer_entity_parser(ptr, path))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_extract_gazetteer_entities_json(
            ptr: *const $crate::CGazetteerEntityParser,
            sentence: *const ::libc::c_char,
            filter_entity_kinds: *const ::ffi_utils::CStringArray,
            results: *mut *const ::libc::c_char,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::extract_gazetteer_entity_json(
                ptr,
                sentence,
                filter_entity_kinds,
                results
            ))
        }

        #[no_mangle]
        pub extern "C" fn snips_nlu_ontology_destroy_gazetteer_entity_parser(
            ptr: *mut $crate::CGazetteerEntityParser,
        ) -> ::ffi_utils::SNIPS_RESULT {
            wrap!($crate::destroy_gazetteer_entity_parser(ptr))
        }
    }
}
