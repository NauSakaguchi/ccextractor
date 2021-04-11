// Define possible exit codes that will be passed on to the fatal function
/* Exit codes. Take this seriously as the GUI depends on them.
   0 means OK as usual,
   <100 means display whatever was output to stderr as a warning
   >=100 means display whatever was output to stdout as an error
*/
pub const EXIT_OK: i32 =                                 0;
pub const EXIT_NO_INPUT_FILES: i32 =                     2;
pub const EXIT_TOO_MANY_INPUT_FILES: i32 =               3;
pub const EXIT_INCOMPATIBLE_PARAMETERS: i32 =            4;
pub const EXIT_UNABLE_TO_DETERMINE_FILE_SIZE: i32 =      6;
pub const EXIT_MALFORMED_PARAMETER: i32 =                7;
pub const EXIT_READ_ERROR: i32 =                         8;
pub const EXIT_WITH_HELP: i32 =                          9;
pub const EXIT_NO_CAPTIONS: i32 =                        10;
pub const EXIT_NOT_CLASSIFIED: i32 =                     300;
pub const EXIT_ERROR_IN_CAPITALIZATION_FILE: i32 =       501;
pub const EXIT_BUFFER_FULL: i32 =                        502;
pub const EXIT_MISSING_ASF_HEADER: i32 =                 1001;
pub const EXIT_MISSING_RCWT_HEADER: i32 =                1002;

pub const CCX_COMMON_EXIT_FILE_CREATION_FAILED: i32 =   5;
pub const CCX_COMMON_EXIT_UNSUPPORTED: i32 =            9;
pub const EXIT_NOT_ENOUGH_MEMORY: i32 =                 500;
pub const CCX_COMMON_EXIT_BUG_BUG: i32 =                1000;

pub const CCX_OK: i32 =       0;
pub const CCX_FALSE: i32 =    0;
pub const CCX_TRUE: i32 =     1;
pub const CCX_EAGAIN: i32 =  -100;
pub const CCX_EOF: i32 =     -101;
pub const CCX_EINVAL: i32 =  -102;
pub const CCX_ENOSUPP: i32 = -103;
pub const CCX_ENOMEM: i32 =  -104;