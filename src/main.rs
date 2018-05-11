
mod list;


#[macro_use] use list::list::List;

fn main() {

	macro_rules! dudu {
	    ($i: ident, $e: expr ) => {
	    	println!("macros for the win");
	    	let $i = {
	    		let a = $e;
	    		println!("{:?}",a );
	    		a
	    	};
		}
	}
	macro_rules! vec_strs {
    (
        // Start a repetition:
        $(
            // Each repeat must contain an expression...
            $element:expr
        )
        // ...separated by commas...
        ,
        // ...zero or more times.
        *
    ) => {
        // Enclose the expansion in a block so that we can use
        // multiple statements.
        {
            let mut v = Vec::new();

            // Start a repetition:
            $(
                // Each repeat will contain the following statement, with
                // $element replaced with the corresponding expression.
                v.push(format!("{}", $element));
            )*

            v
        }
	    };
	}



	dudu!(x, 3 * 3);
	println!("{:?}",x );

	let t = vec_strs!["this", "that"];
	println!("{:?}", t);



	let li : List<u8> = List![1, 2, 3];

	println!("{:?}", li);
}








