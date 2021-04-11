pub struct ccx_s_options
{
   test: i32
}

impl ccx_s_options
{
    pub fn new() -> Self
    {
        let ccx_options:ccx_s_options = Self
        {
            test: 10
        };
        ccx_options
    }

}