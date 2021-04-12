/* CCExtractor, originally by carlos at ccextractor.org, now a lot of people.
Credits: See AUTHORS.TXT
License: GPL 2.0
*/

use std::env;
use ccextractor::ccextractor::*;

//call ccextractor.rs here which works like ccextractor.c / .h

/*memo*/
//ccx_options (global variable in C program)
//
//the structure of directory (suggestion) (Refactor)
//lib_ccx - header - ccx_common_options.rs (ccx_common_options.h)
//        |
//        - body - ccx_common_options.rs (ccx_common_options.c)

fn main()
{
    let args: Vec<String> = env::args().collect(); // get input as command arguments

    ccextractor(args);
}