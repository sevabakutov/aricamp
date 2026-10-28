use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_win_scenario() {
    let mut cmd = Command::cargo_bin("task_3_1").unwrap();

    cmd.arg("42")
       .write_stdin("10\n52\n42\n")
       .assert()
       .success()
       .stdout(predicate::str::contains("Too small!"))
       .stdout(predicate::str::contains("Too big!"))
       .stdout(predicate::str::contains("You win!"));
}
