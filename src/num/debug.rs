
#[macro_export]
macro_rules! debug_tree {
	[ @[$($a:tt)*]: @[$($b:tt)*]: $($add_str:literal, )? a , $n: ident ! $(($($args:tt)*))? ] => {{
		println!(
			concat!($($add_str,)? "a: {:?}"), stringify!($($a)*)
		);
		$n ! {
			@[$($a)*]: @[$($b)*]:
			$($($args)*)?
		}
	}};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $($add_str:literal, )? b , $n: ident ! $(($($args:tt)*))? ] => {{
		println!(
			concat!($($add_str,)? "b: {:?}"), stringify!($($b)*)
		);
		$n ! {
			@[$($a)*]: @[$($b)*]:
			$($($args)*)?
		}
	}};
	
	[ @[$($a:tt)*]: @[$($b:tt)*]: $($add_str:literal, )? $(a, b)? $((a, b))? , $n: ident ! $(($($args:tt)*))? ] => {{
		println!(
			concat!($($add_str,)? "a: {:?}, b: {:?}"), stringify!($($a)*), stringify!($($b)*)
		);
		$n ! {
			@[$($a)*]: @[$($b)*]:
			
			$($($args)*)?
		}
	}};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $(a, b)? $((a, b))? $(, $n: ident ! $(($($args:tt)*))?)? ] => {{
		println!(
			"a: {:?}, b: {:?}", stringify!($($a)*), stringify!($($b)*)
		);
		$($n ! {
			@[$($a)*]: @[$($b)*]:
			
			$($($args)*)?
		})?
	}};
	[  ] => {
		println!("zero");
	};
}

#[macro_export]
macro_rules! debug_tree_nums {
	[ @[$($a:tt)*]: @[$($b:tt)*]: $($add_str:literal, )? a , $n: ident ! $(($($args:tt)*))? ] => {{
		println!(
			concat!($($add_str,)? "a: {:?}"), into_num!( @[$($a)*]: @[$($b)*]: a )
		);
		$n ! {
			@[$($a)*]: @[$($b)*]:
			$($($args)*)?
		}
	}};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $($add_str:literal, )? b , $n: ident ! $(($($args:tt)*))? ] => {{
		println!(
			concat!($($add_str,)? "b: {:?}"), into_num!( @[$($a)*]: @[$($b)*]: b )
		);
		$n ! {
			@[$($a)*]: @[$($b)*]:
			$($($args)*)?
		}
	}};
	
	[ @[$($a:tt)*]: @[$($b:tt)*]: $($add_str:literal, )? $(a, b)? $((a, b))? , $n: ident ! $(($($args:tt)*))? ] => {{
		println!(
			concat!($($add_str,)? "a: {:?}, b: {:?}"), into_num!( @[$($a)*]: @[$($b)*]: a ), into_num!( @[$($a)*]: @[$($b)*]: b )
		);
		$n ! {
			@[$($a)*]: @[$($b)*]:
			
			$($($args)*)?
		}
	}};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $(a, b)? $((a, b))? $(, $n: ident ! $(($($args:tt)*))?)? ] => {{
		println!(
			"a: {:?}, b: {:?}", into_num!( @[$($a)*]: @[$($b)*]: a ), into_num!( @[$($a)*]: @[$($b)*]: b )
		);
		$($n ! {
			@[$($a)*]: @[$($b)*]:
			
			$($($args)*)?
		})?
	}};
	[  ] => {
		println!("zero");
	};
}
