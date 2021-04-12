use crate::lib_ccx::ccx_common_option::ccx_s_options;
use crate::lib_ccx::ccx_common_common::*;

impl ccx_s_options {
    pub fn parse_parameters(&mut self, command_args:Vec<String>) -> i32
    {
        for command_arg in &command_args
        {
            match &**command_arg {
                "--help" | "-h" => {
                    // print_usage();
                    return EXIT_WITH_HELP;
                }
                // unimplemented!();
                _ => {
                    // unimplemented!();
                }
            }
        }
        // unimplemented!();
        //need to implement later (see the documentation)

        if command_args.len() == 1
        {
            // unimplemented!();
            return EXIT_NO_INPUT_FILES;
        }
        -1 // just return temp value
    }
}
