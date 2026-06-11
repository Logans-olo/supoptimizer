extern crate egg;

use egg::{*, rewrite as rw};

define_language! {
    enum SimpLang { 
        Num(i32),
        "+" = Add([Id;2]),
        "-" = Sub([Id;2]),
        "*" = Mul([Id;2]),
        Symbol(Symbol),
    }

}

fn make_rules() -> Vec<Rewrite<SimpLang, ()>> {
    vec![
        rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rewrite!("add-0"; "(+ ?a 0)" => "?a"),
        rewrite!("mul-0"; "(* ?a 0)" => "0"),
        rewrite!("mul-1"; "(* ?a 1)" => "?a"),
    ]
}



fn main() {}

fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<SimpLang> = s.parse().unwrap();

    
    let runner = Runner::default().with_expr(&expr).run(&make_rules());

    let root = runner.roots[0];

    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best) = extractor.find_best(root);
    println!("Simplified {} to {} with cost {}", expr, best, best_cost);
    best.to_string()
}


#[test]
fn simple_test() { 
    assert_eq!(simplify("(* 0 42)"), "0");
    assert_eq!(simplify("(+ 0 (* 1 foo))"), "foo");

}
