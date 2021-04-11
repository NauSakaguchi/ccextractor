use crate::lib_ccx::ccx_common_option::ccx_s_options;
use crate::lib_ccx::ccx_common_common::*;
use std::process::exit;

pub fn ccextractor(args: Vec<String>)
{
    // setlocale(LC_ALL, ""); // Supports non-English CCs
    //may be implemented later (see the documentation)

    let mut ccx_options:ccx_s_options = ccx_s_options::new(); //unimplemented!()
    //::new() instead of api_init_options() & init_options()
    //camel case or snake case?
    //not using a global variable for ccx_options (see the documentation)

    ccx_options.parse_configuration(); //unimplemented!()
    // If "ccextractor.cnf" is present, takes options from it.
    // See docs/ccextractor.cnf.sample for more info.

    let compile_ret:i32 = ccx_options.parse_parameters(args); //unimplemented!()

    if compile_ret == EXIT_NO_INPUT_FILES
    {
        println!("No Input"); //temp implement
        exit(compile_ret); //temp implement
        // print_usage();
        // fatal(EXIT_NO_INPUT_FILES, "(This help screen was shown because there were no input files)\n");
    }
    else if compile_ret == EXIT_WITH_HELP
    {
        println!("EXIT_WITH_HELP"); // temp implement
        return; // return EXIT_OK;
    }
    else if compile_ret != EXIT_OK
    {
        println!("NOT EIXT_OK"); // temp implement
        exit(compile_ret);
    }

    println!("EIXT_OK"); //temp implement
    // int start_ret = api_start(*api_options);
    // return start_ret;
}