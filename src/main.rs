use std::{
    fs::File,
    io::{BufRead, BufReader, LineWriter, Write},
    process::{Command, Stdio},
};

fn main() {
    let fiji_path = "/Users/sc13967/Applications/Fiji_(JDK21)/fiji";
    let workflow_path = "/Users/sc13967/Desktop/TEST_batch.mia";
    let input_path = "/Users/sc13967/Desktop/batchtest/";
    let permutation_path = "/Users/sc13967/Desktop/permutations.txt";

    let file = File::create(permutation_path).expect("Can't open file");
    let mut file = LineWriter::new(file);

    let mut cp_args: Vec<Vec<&str>> = Vec::new();
    cp_args.push(vec!["", "--norm_percentile,0,100"]);
    cp_args.push(vec![
        "",
        "--niter,100",
        "--niter,500",
        "--niter,2000",
    ]);

    let mut cp_permutations: Vec<String> = Vec::new();
    iterate_cp_args(&cp_args, 0, "", &mut cp_permutations);

    let mut variables: Vec<(String, Vec<String>)> = Vec::new();
    variables.push(("CPModel".into(), vec!["cpsam_v2".into(), "cpdino".into(), "cpdino-vitb".into()]));
    variables.push(("CPDiameter".into(), vec!["30".into(), "50".into(), "70".into(), "90".into()]));
    variables.push(("CPFlow".into(), vec!["0.4".into(), "0.6".into(), "0.8".into()]));
    variables.push(("CPAnisotropy".into(), vec!["1".into(), "9.7".into()]));
    variables.push(("CPArgs".into(), cp_permutations));

    let mut parameter_permutations: Vec<String> = Vec::new();
    iterate_parameters(&variables, 0, "", &mut parameter_permutations);

    let mut count = 1;
    for permutation in parameter_permutations {
        println!("ID {}: {}", count, permutation);
        count += 1;
    }
    
    // let mut count = 1;
    // for parameter_permutation in parameter_permutations {
    //     file.write_all(format!("ID:{};{}\r", count, parameter_permutation).as_bytes())
    //         .expect("Can't write line");
    //     let _ = file.flush();

    //     run_fiji(
    //         fiji_path,
    //         workflow_path,
    //         input_path,
    //         parameter_permutation,
    //         count,
    //     );
    //     count += 1;
    // }
}

fn iterate_parameters(
    variables: &Vec<(String, Vec<String>)>,
    level: usize,
    cmd: &str,
    param_permutations: &mut Vec<String>,
) {
    for i in 0..variables[level].1.len() {
        let cmd_out = format!("{}{}:{}", cmd, variables[level].0, variables[level].1[i]);
        if level < variables.len() - 1 {
            iterate_parameters(
                variables,
                level + 1,
                &format!("{};", cmd_out),
                param_permutations,
            );
        } else {
            param_permutations.push(cmd_out);
        }
    }
}

fn iterate_cp_args(
    cp_args: &Vec<Vec<&str>>,
    level: usize,
    cmd: &str,
    cp_permutations: &mut Vec<String>,
) {
    for i in 0..cp_args[level].len() {
        let cmd_out = format!("{}{}", cmd, cp_args[level][i]);
        let mut comma = "";

        if cp_args[level][i].len() > 0 {
            comma = ",";
        }

        if level < cp_args.len() - 1 {
            iterate_cp_args(
                cp_args,
                level + 1,
                &format!("{}{}", cmd_out, comma),
                cp_permutations,
            );
        } else if cp_args[level][i].len() > 0 {
            cp_permutations.push(cmd_out);
        }
    }
}

fn run_fiji(
    fiji_path: &str,
    workflow_path: &str,
    input_path: &str,
    parameters: String,
    count: u32,
) {
    let mia_arg = format!(
        "workflowPath='{}', inputPath='{}', variables='{};ID:{}'",
        workflow_path, input_path, parameters, count
    );

    println!("{}", mia_arg);

    let mut child = Command::new(fiji_path)
        .arg("--headless")
        .arg("--run")
        .arg("MIA (headless)")
        .arg(mia_arg)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run");

    let result = child.stdout.take().unwrap();
    let reader = BufReader::new(result);
    for line in reader.lines() {
        println!("{}", line.expect("Can't read line"));
    }

    let status = child.wait().expect("Failed to wait for Fiji");
    println!("Fiji finished with status: {}", status);
}
