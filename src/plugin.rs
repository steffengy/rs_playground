#![feature(plugin_registrar, rustc_private)]

extern crate aster;
extern crate syntax;
extern crate rustc_plugin;

use aster::AstBuilder;
use rustc_plugin::Registry;
use syntax::ast::{Item_, MetaItem};
use syntax::ext::base::ExtCtxt;
use syntax::codemap::Span;
use syntax::ext::base::Annotatable;
use syntax::ext::base::SyntaxExtension::MultiDecorator;
use syntax::parse::token::intern;

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("test_plug"), MultiDecorator(Box::new(test_expand)));
}

fn test_expand(_: &mut ExtCtxt, _: Span, _: &MetaItem, anno: &Annotatable, push: &mut FnMut(Annotatable)) {
    let builder = AstBuilder::new();

    let block = builder.block()
        .stmt().expr().call()
            .path().id("hello_world").build()
            .build()
        .build();

    let fn_ = builder.item().fn_("zif_hello_world")
        .default_return()
        .build(block);

    println!("{:?}", syntax::print::pprust::item_to_string(&*fn_)); // "fn zif_hello_world() { hello_world(); }"
    push(Annotatable::Item(fn_))
}
