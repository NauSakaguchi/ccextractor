pub struct ccx_s_options
{
    test: isize
}

impl ccx_s_options
{
    pub fn new() -> ccx_s_options
    {
        let ccx_s_options = Self
        {
            test: 10
        };
        ccx_s_options
    }

}