/*
MIT License

Copyright (c) 2017 Viorel Bota

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::fs::OpenOptions;
use std::io::prelude::*;

const APPLICATIONVERSION: &'static str = env!("CARGO_PKG_VERSION");
const APPLICATIONNAME: &'static str = env!("CARGO_PKG_NAME");

pub fn get_about_text () -> String
{           
        format! ("{} Version: {}\n{}", APPLICATIONNAME, APPLICATIONVERSION, get_all_licences() )
}

fn get_all_licences () -> String
{        
        format! ("Licence: \n{}\n3'rd party licences:\n{}", get_own_licence (), get_3rd_party_licence () )
}

fn get_own_licence () -> String
{
                String::from (
"\tCopyright (c) 2017 Viorel Bota\n
\n
\tPermission is hereby granted, free of charge, to any person obtaining a copy\n
\tof this software and associated documentation files (the \"Software\"), to deal\n
\tin the Software without restriction, including without limitation the rights\n
\tto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n
\tcopies of the Software, and to permit persons to whom the Software is\n
\tfurnished to do so, subject to the following conditions:\n
\n
\tThe above copyright notice and this permission notice shall be included in all\n
\tcopies or substantial portions of the Software.\n
\n
\tTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n
\tIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n
\tFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n
\tAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n
\tLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n
\tOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\n
\tSOFTWARE."
                )
}

fn get_3rd_party_licence () -> String
{
                String::from (
"\tfor chronotope/chrono used in time.rs:\n
\tCopyright (c) 2014, Kang Seonghoon.\n
\n
\tPermission is hereby granted, free of charge, to any person obtaining a copy\n
\tof this software and associated documentation files (the \"Software\"), to deal\n
\tin the Software without restriction, including without limitation the rights\n
\tto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n
\tcopies of the Software, and to permit persons to whom the Software is\n
\tfurnished to do so, subject to the following conditions:\n
\n
\tThe above copyright notice and this permission notice shall be included in\n
\tall copies or substantial portions of the Software.\n
\n
\tTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n
\tIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n
\tFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n
\tAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n
\tLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n
\tOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN\n
\tTHE SOFTWARE."
                )
}
