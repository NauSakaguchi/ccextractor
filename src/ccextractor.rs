use crate::lib_ccx::ccx_common_option::ccx_s_options;
use crate::lib_ccx::ccx_common_common::*;
use crate::lib_ccx::configuration::parse_configuration;
use crate::lib_ccx::params::parse_parameters;
use std::process::exit;

pub fn ccextractor(args: Vec<String>)
{
    // setlocale(LC_ALL, ""); // Supports non-English CCs
    //外部関数のためどうすればいいのかよくわからん

    let api_options:ccx_s_options = ccx_s_options::new(); //unimplemented!()
    //グローバル変数はRustの流儀にあってないため使わない
    //名前がキャメルケース推奨

    parse_configuration(&api_options); //unimplemented!()
    // If "ccextractor.cnf" is present, takes options from it.
    // See docs/ccextractor.cnf.sample for more info.

    let compile_ret:isize = parse_parameters(&api_options, args); //unimplemented!()

    if compile_ret == EXIT_NO_INPUT_FILES
    {
        println!("No Input"); //temp
        exit(compile_ret as i32); //temp
        // print_usage();
        // fatal(EXIT_NO_INPUT_FILES, "(This help screen was shown because there were no input files)\n");
    }
    else if compile_ret == EXIT_WITH_HELP
    {
        println!("EXIT_WITH_HELP"); // temp
        return; // return EXIT_OK;
    }
    else if compile_ret != EXIT_OK
    {
        println!("NOT EIXT_OK"); // temp
        exit(compile_ret as i32);
    }

    println!("EIXT_OK"); //temp
    // int start_ret = api_start(*api_options);
    // return start_ret;
}