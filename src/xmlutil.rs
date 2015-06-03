use std::collections::HashMap;

use sxd_document::Package;
use sxd_document::dom::Document;
use sxd_document::parser::Parser;
use sxd_xpath::{Value, Functions, Variables, Namespaces, Factory, EvaluationContext, Expression};


pub fn parse(xml: &str) -> Package {
    Parser::new().parse(xml).unwrap()
}

pub fn evaluate<'d>(package: &'d Document<'d>, xpath: &str) -> Value<'d> {
    let evaluator = XPathEvaluator::new();
    evaluator.evaluate(package, xpath)
}

struct XPathEvaluator<'d> {
    functions: Functions,
    variables: Variables<'d>,
    namespaces: Namespaces,
    factory: Factory,
}

impl<'d> XPathEvaluator<'d> {
    fn new() -> XPathEvaluator<'d> {
        let mut fns = HashMap::new();
        super::sxd_xpath::function::register_core_functions(&mut fns);
        XPathEvaluator {
            functions: fns,
            variables: HashMap::new(),
            namespaces: HashMap::new(),
            factory: Factory::new(),
        }
    }

    fn evaluate(&self, doc: &'d Document<'d>, xpath: &str) -> Value<'d> {
        let root = doc.root();
        let context = EvaluationContext::new(
            root,
            &self.functions,
            &self.variables,
            &self.namespaces,
        );

        let xpath = self.factory.build(xpath).unwrap().unwrap();
        xpath.evaluate(&context).ok().unwrap()
    }
}
