use crate::repl::tests::{TestResult, run_test};

#[test]
fn if_test1() -> TestResult {
    run_test("if true { 10 } else { 20 } ", "10")
}

#[test]
fn if_test2() -> TestResult {
    run_test("if false { 10 } else { 20 } ", "20")
}

#[test]
fn simple_if() -> TestResult {
    run_test("if true { 10 } ", "10")
}

#[test]
fn simple_if2() -> TestResult {
    run_test("if false { 10 } ", "")
}

#[test]
fn if_cond() -> TestResult {
    run_test("if 2 < 3 { 3 } ", "3")
}

#[test]
fn if_cond2() -> TestResult {
    run_test("if 2 > 3 { 3 } ", "")
}

#[test]
fn if_cond3() -> TestResult {
    run_test("if 2 < 3 { 5 } else { 4 } ", "5")
}

#[test]
fn if_cond4() -> TestResult {
    run_test("if 2 > 3 { 5 } else { 4 } ", "4")
}

#[test]
fn if_elseif1() -> TestResult {
    run_test("if 2 > 3 { 5 } else if 6 < 7 { 4 } ", "4")
}

#[test]
fn if_elseif2() -> TestResult {
    run_test("if 2 < 3 { 5 } else if 6 < 7 { 4 } else { 8 } ", "5")
}

#[test]
fn if_elseif3() -> TestResult {
    run_test("if 2 > 3 { 5 } else if 6 > 7 { 4 } else { 8 } ", "8")
}

#[test]
fn if_elseif4() -> TestResult {
    run_test("if 2 > 3 { 5 } else if 6 < 7 { 4 } else { 8 } ", "4")
}

#[test]
fn mutation_in_else() -> TestResult {
    run_test(
        "mut x = 100; if 2 > 3 { $x = 200 } else { $x = 300 }; $x ",
        "300",
    )
}

#[test]
fn mutation_in_else2() -> TestResult {
    run_test(
        "mut x = 100; if 2 > 3 { $x = 200 } else if true { $x = 400 } else { $x = 300 }; $x ",
        "400",
    )
}
