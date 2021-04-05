use std::env;
use ccextractor::ccextractor::*;

//main.rs から ccextractor.rsを呼び出す
//match -> if

/**設計方針の変更**/
//キャメルに統一します
//constなら意味があるけど、ないならグローバル変数を直接いじるだけで良いのでは？


fn main() {
    let args: Vec<String> = env::args().collect(); // get input as command arguments

    ccextractor(args);

    // // setlocale(LC_ALL, ""); // Supports non-English CCs
    // //外部関数のためどうすればいいのかよくわからん
    //
    // let api_options:ccx_s_options = ccx_s_options::new();
    // //グローバル変数はRustの流儀にあってないため使わない
    // //名前がキャメルケース推奨
    //
    // parse_configuration(&api_options);
    // // If "ccextractor.cnf" is present, takes options from it.
    // // See docs/ccextractor.cnf.sample for more info.
    //
    // let compile_ret:isize = parse_parameters(&api_options, args);
    //
    // match compile_ret {
    //      EXIT_NO_INPUT_FILES => {
    //         println!("No Input");
    //          // print_usage();
    //          // fatal(EXIT_NO_INPUT_FILES, "(This help screen was shown because there were no input files)\n");
    //     }
    //
    //     EXIT_WITH_HELP => {
    //         println!("EXIT_WITH_HELP");
    //         return;
    //     }
    //
    //     EXIT_BUFFER_FULL => {
    //
    //     }
    //
    //     EXIT_OK => {
    //         println!("EIXT_OK");
    //         // int start_ret = api_start(*api_options);
    //         // return start_ret;
    //     }
    //
    //     _ => {
    //         println!("not EIXT_OK");
    //         exit(compile_ret as i32);
    //     }
    // }
}

