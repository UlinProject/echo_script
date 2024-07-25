
#[macro_export]
macro_rules! insert {
	[ @[$($a:tt)*]: @[$($b:tt)*]: a, b = [ $($new:tt)* ], $n: ident ! $(($($args:tt)*))? ] => {
		$n! {
			@[$($new)*]:
			@[$($new)*]: 
			
			$($($args)*)?
		}
	};
	
	[ @[$($a:tt)*]: @[$($b:tt)*]: a += [ $($new_a:tt)* ], $n: ident ! $(($($args:tt)*))? ] => {
		$n! {
			@[$($a)* $($new_a)*]: 
			@[$($b)*]: 
			
			$($($args)*)?
		}
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: b += [ $($new_b:tt)* ], $n: ident ! $(($($args:tt)*))? ] => {
		$n! {
			@[$($a)*]: 
			@[$($b)* $($new_b)*]: 
			
			$($($args)*)?
		}
	};
	
	[ @[$($a:tt)*]: @[$($b:tt)*]: a = [ $($new_a:tt)* ], $n: ident ! $(($($args:tt)*))? ] => {
		$n! {
			@[$($new_a)*]:
			@[$($b)*]: 
			
			$($($args)*)?
		}
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: b = [ $($new_b:tt)* ], $n: ident ! $(($($args:tt)*))? ] => {
		$n ! {
			@[$($a)*]:
			@[$($new_b)*]:
			
			$($($args)*)?
		}
	};
}

