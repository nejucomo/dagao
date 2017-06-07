#[macro_export]
macro_rules! res_unwrap {
    ( $x:expr ) => {{
        let result = $x;
        assert!(result.is_ok(), stringify!($x));
        result.unwrap()
    }}
}


macro_rules! test_with_fs {

    ($name:ident $closure:expr) => {
        #[test]
        fn $name () {
            let testpathbuf = {
                use std::{env,fs};
                use hashstore::UniqueValue;

                let mut it = module_path!().split("::");
                let mut pb = env::temp_dir();

                {
                    let mut first = String::from("test-");
                    first.push_str(it.next().unwrap());
                    first.push_str(".");
                    first.push_str(
                        &res_unwrap!(UniqueValue::generate())
                            .encoded());

                    pb.push(first);
                }

                for part in it {
                    pb.push(part);
                }
                res_unwrap!(fs::create_dir_all(pb.as_path()));

                pb.push(stringify!($name));

                pb
            };

            let cl = $closure;

            cl(testpathbuf.as_path());
        }
    }
}


#[macro_export]
macro_rules! tests_with_fs {
    ( $( $name:ident $closure:expr );* ) => {
        $( test_with_fs!( $name $closure ); )*
    }
}
