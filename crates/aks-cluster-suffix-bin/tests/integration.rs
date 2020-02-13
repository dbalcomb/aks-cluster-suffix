use assert_cmd::Command;
use serde::Serialize;

#[derive(Serialize)]
struct Input {
    input: String,
}

#[derive(Serialize)]
struct Output {
    output: String,
}

#[test]
fn test_input_error() {
    Command::cargo_bin("aks-cluster-suffix")
        .unwrap()
        .assert()
        .code(1);
}

#[test]
fn test_input_output() {
    let input = Input {
        input: String::from("k8stest"),
    };

    let output = Output {
        output: String::from("21324540"),
    };

    Command::cargo_bin("aks-cluster-suffix")
        .unwrap()
        .write_stdin(serde_json::to_string(&input).unwrap())
        .assert()
        .success()
        .stdout(serde_json::to_string(&output).unwrap());
}
