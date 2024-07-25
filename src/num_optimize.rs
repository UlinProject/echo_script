
//
#[macro_export]
macro_rules! num_optimize {
	// end. a
	[
		@num_optimize[$($a:tt)*]
		@end[ a, [$($all:tt)*], $n: ident ! $(($($args:tt)*))? ]
	] => {
		$n! {
			@[ $($a)* ]: @[ $($all)* ]: $($($args)*)?
		}
	};
	// end. b
	[
		@num_optimize[$($a:tt)*]
		@end[ b, [$($all:tt)*], $n: ident ! $(($($args:tt)*))? ]
	] => {
		$n! {
			@[ $($all)* ]: @[ $($a)* ]: $($($args)*)?
		}
	};
	
	// optimize_rules
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[+][+][+][+][+][+][-][-][-][-][-][-] // OPTIMIZE_RULE
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[+][+][+][+][+][-][-][-][-][-] // OPTIMIZE_RULE
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[+][+][+][+][-][-][-][-] // OPTIMIZE_RULE
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[+][+][+][-][-][-] // OPTIMIZE_RULE
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[+][+][-][-] // OPTIMIZE_RULE
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[+][-] // OPTIMIZE_RULE
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[+] // SKIP
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* [+] ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	[
		@num_optimize[$($all:tt)*]
		@end[ $($end:tt)* ]
		[-] // SKIP
		
		$($next:tt)*
	] => {
		$crate::num_optimize! {
			@num_optimize[ $($all)* [-] ]
			@end[ $($end)* ]
			
			$($next)*
		}
	};
	
	// in a or in b
	[ @[$($a:tt)*]: @[$($b:tt)*]: a, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::num_optimize! {
			@num_optimize[]
			@end[ a, [$($b)*], $n ! $(($($args)*))? ]
			
			$($a)*
		}
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: b, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::num_optimize! {
			@num_optimize[]
			@end[ b, [$($a)*], $n ! $(($($args)*))? ]
			
			$($b)*
		}
	};
}