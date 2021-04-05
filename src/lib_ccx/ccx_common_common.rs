// Define possible exit codes that will be passed on to the fatal function
/* Exit codes. Take this seriously as the GUI depends on them.
   0 means OK as usual,
   <100 means display whatever was output to stderr as a warning
   >=100 means display whatever was output to stdout as an error
*/
pub const EXIT_OK: isize =                                 0;
pub const EXIT_NO_INPUT_FILES: isize =                     2;
pub const EXIT_TOO_MANY_INPUT_FILES: isize =               3;
pub const EXIT_INCOMPATIBLE_PARAMETERS: isize =            4;
pub const EXIT_UNABLE_TO_DETERMINE_FILE_SIZE: isize =      6;
pub const EXIT_MALFORMED_PARAMETER: isize =                7;
pub const EXIT_READ_ERROR: isize =                         8;
pub const EXIT_WITH_HELP: isize =                          9;
pub const EXIT_NO_CAPTIONS: isize =                        10;
pub const EXIT_NOT_CLASSIFIED: isize =                     300;
pub const EXIT_ERROR_IN_CAPITALIZATION_FILE: isize =       501;
pub const EXIT_BUFFER_FULL: isize =                        502;
pub const EXIT_MISSING_ASF_HEADER: isize =                 1001;
pub const EXIT_MISSING_RCWT_HEADER: isize =                1002;

pub const CCX_COMMON_EXIT_FILE_CREATION_FAILED: isize =   5;
pub const CCX_COMMON_EXIT_UNSUPPORTED: isize =            9;
pub const EXIT_NOT_ENOUGH_MEMORY: isize =                 500;
pub const CCX_COMMON_EXIT_BUG_BUG: isize =                1000;

pub const CCX_OK: isize =       0;
pub const CCX_FALSE: isize =    0;
pub const CCX_TRUE: isize =     1;
pub const CCX_EAGAIN: isize =  -100;
pub const CCX_EOF: isize =     -101;
pub const CCX_EINVAL: isize =  -102;
pub const CCX_ENOSUPP: isize = -103;
pub const CCX_ENOMEM: isize =  -104;