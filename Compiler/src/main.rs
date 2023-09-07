#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;

use rustc_ast::ast;
use rustc_driver::Compilation;
use rustc_hir::LangItem;
use rustc_interface::interface;
use rustc_session::config::Options;

fn main() {
    let mut rustc_args = std::env::args().collect::<Vec<_>>();
    rustc_args.insert(1, "collect_lang_item".to_string());

    let options = Options {
        crate_types: vec![rustc_session::config::CrateType::Rlib],
        output_dir: None,
        ..Options::default()
    };

    let mut callbacks = rustc_driver::Callbacks::new();
    callbacks.after_analysis = Some(Box::new(|state| {
        let krate = state.hir_crate.unwrap();
        let lang_items = krate.session.lang_items();
        
        if let Some(lang_item) = lang_items.require(LangItem::Option) {
            println!("Found LangItem::Option at {:?}", lang_item);
        } else {
            println!("LangItem::Option not found");
        }
    }));

    rustc_driver::run_compiler_with_args(
        &rustc_args,
        &options,
        &mut callbacks,
        None,
    ).unwrap();
}
