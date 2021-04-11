/* CCExtractor, originally by carlos at ccextractor.org, now a lot of people.
Credits: See AUTHORS.TXT
License: GPL 2.0
*/

use std::env;
use ccextractor::ccextractor::*;


/**memo**/
//camel case or snake case?
//call ccextractor.rs here which works like ccextractor.c / .h

fn main()
{
    let args: Vec<String> = env::args().collect(); // get input as command arguments

    ccextractor(args);
}