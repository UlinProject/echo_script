
#[macro_export]
macro_rules! into_num {
	[] => {0}; // end

	[ // end
		@into_num [ $n:ident ];
	] => {{
		$n
	}};
	
	[
		@into_num [ $n:ident ];
		[ $rule:tt ] $($unk:tt)*
	] => {
		{
			#[allow(clippy::assign_op_pattern)] {
				$n = $n $rule 1;
			}
		}
		$crate::into_num! {
			@into_num[$n];
			$($unk)*
		}
	};
	
	[ // in
		@[$( [$($a:tt)?] )*]: @[$( [$($b:tt)?] )*]: a, b
	] => {{
		let mut a = 0;
		let mut b = 0;
		{
			a = $crate::into_num! {
				@into_num [a];
				$( [$($a)?] )*
			};
			b = $crate::into_num! {
				@into_num [b];
				$( [$($b)?] )*
			};
		}
		
		(a, b)
	}};
	[ // in
		@[$( [$($a:tt)?] )*]: @[$( [$($b:tt)?] )*]: a
	] => {{
		let mut a = 0;

		$crate::into_num! {
			@into_num [a];
			$( [$($a)?] )*
		}
	}};
	[ // in
		@[$( [$($a:tt)?] )*]: @[$( [$($b:tt)?] )*]: b
	] => {{
		let mut b = 0;
		
		$crate::into_num! {
			@into_num [b];
			$( [$($b)?] )*
		}
	}};
}
