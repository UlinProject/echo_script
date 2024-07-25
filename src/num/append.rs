
#[macro_export]
macro_rules! append {
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 10, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p][$p][$p][$p][$p][$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 9, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p][$p][$p][$p][$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 8, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p][$p][$p][$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 7, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p][$p][$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 6, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p][$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 5, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 4, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 3, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 2, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p][$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 1, $n: ident ! $(($($args:tt)*))? ] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$nt += [ [$p] ],
			$n ! $(( $($args)* ))?
		)
	};
	[ @[$($a:tt)*]: @[$($b:tt)*]: $nt:tt $p:tt 0, $n: ident ! $(( $($args:tt)* ))? ] => {
		$n! {
			@[$($a)*]:
			@[$($b)*]:
			
			$($($args)*)?
		}
	};
}
