
mod into_num;
mod num_optimize;
mod insert;

pub mod num {
	pub mod append;
	pub mod debug;
	pub mod cfg;
	pub mod r#while;
}

#[macro_export]
macro_rules! one_line_expr {
	[] => {};
	[
		{
			$($all:tt)*
		}($a: tt, $b: tt);
	] => {
		$crate::__one_line_expr!(
			#init_stack($a, $b);
			
			$($all)*
		)
	};
	[
		{
			$($all:tt)*
		}();
	] => {
		$crate::one_line_expr!(
			{
				$($all)*
			}(0, 0);
		)
	};
	[
		$($all:tt)+
	] => {
		$crate::one_line_expr!(
			{
				$($all)*
			}(0, 0);
		)
	};
	
}

#[macro_export]
macro_rules! __one_line_expr {
	[
		$(@[$($_a:tt)*]: @[$($_b:tt)*]:)?
	] => {};
	[
		#init_stack($a: tt, $b: tt);
		
		$($all:tt)*
	] => {
		$crate::insert!(
			@[]: @[]:
			a, b = [],
			
			$crate::append!(
				a + $a,
				
				$crate::append!(
					b + $b,
					
					__one_line_expr!($($all)*)
				)
			)
		)
	};
	
	[
		$(@[$($_a:tt)*]: @[$($_b:tt)*]:)?
		
		#set_stack( @[$($a:tt)*]: @[$($b:tt)*]: );
		$($all:tt)*
	] => {
		$crate::__one_line_expr!( @[$($a)*]: @[$($b)*]: $($all)* )
	};
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		
		#set_var($v:tt, $($new_ab:tt)*);
		$($all:tt)*
	] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			$v = [$($new_ab)*],
			
			__one_line_expr!($($all)*)
		)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		
		#set_all_vars($($all_v:tt)*);
		$($all:tt)*
	] => {
		$crate::__one_line_expr!(
			$($all_v)*
			$($all)*
		)
	};
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		
		#( $($insert:tt)* );
		$($all:tt)*
	] => {
		$($insert)*
		$crate::__one_line_expr!(@[$($a)*]: @[$($b)*]: $($all)*)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		
		#{ $($insert:tt)* }
		$($all:tt)*
	] => {
		$($insert)*
		$crate::__one_line_expr!(@[$($a)*]: @[$($b)*]: $($all)*)
	};
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		num_optimize($($args:tt)*);
		
		$($all:tt)*
	] => {
		$crate::num_optimize!(
			@[$($a)*]: @[$($b)*]:
			$($args)*, __one_line_expr!($($all)*)
		)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		if $n:tt $p:tt $v:tt {
			$($true:tt)*
		}else {
			$($false:tt)*
		}
		$($add:tt)*
	] => {
		$crate::num_optimize!(
			@[$($a)*]: @[$($b)*]:
			
			$n, r#if!(if $n $p $v {
				__one_line_expr!($($true)* $($add)*)
			}else {
				__one_line_expr!($($false)* $($add)*)
			})
		)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		if $n:tt $p:tt $v:tt {
			$($true:tt)*
		}
		$($add:tt)*
	] => {
		$crate::num_optimize!(
			@[$($a)*]: @[$($b)*]:
			
			$n, r#if!(if $n $p $v {
				__one_line_expr!($($true)* $($add)*)
			}else {
				__one_line_expr!($($add)*)
			})
		)
	};
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		{ $($in:tt)* }
		$($all:tt)*
	] => {
		$crate::__one_line_expr! (
			@[$($a)*]: @[$($b)*]:
			
			$($in)*
			
			#set_all_vars( @[$($a)*]: @[$($b)*]: );
			$($all)*
		)
	};
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		for $n:tt in $a1:tt .. $a2:tt .rev() {
			$($for:tt)*
		}
		$($all:tt)*
	] => {
		$crate::__one_line_expr! (
			@[$($a)*]: @[$($b)*]:
			
			{
				$n = $a2;
				while $n != $a1 {
					$($for)*
					
					$n -= 1;
				}
			}
			$($all)*
		)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		for $n:tt in $a1:tt .. $a2:tt {
			$($for:tt)*
		}
		$($all:tt)*
	] => {
		$crate::__one_line_expr! (
			@[$($a)*]: @[$($b)*]:
			
			{
				$n = $a1;
				while $n != $a2 {
					$($for)*
					
					$n += 1;
				}
			}
			$($all)*
		)
	};
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		while $n:tt $p:tt $v:tt {
			$($true:tt)*
		}else { $($else:tt)* }
		$($all:tt)*
	] => {
		$crate::num_optimize!(
			@[$($a)*]: @[$($b)*]:
			
			$n, r#while!(while $n $p $v {
				$($true)*
			}else{ $($else)* $($all)* })
		)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		while $n:tt $p:tt $v:tt {
			$($true:tt)*
		}
		$($all:tt)*
	] => {
		$crate::num_optimize!(
			@[$($a)*]: @[$($b)*]:
			
			$n, r#while!(while $n $p $v {
				$($true)*
			}else{ $($all)* })
		)
	};
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		$n:tt += $e:tt;
		
		$($all:tt)*
	] => {
		$crate::append!(
			@[$($a)*]: @[$($b)*]: 
			$n + $e, __one_line_expr!($($all)*)
		)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		$n:tt -= $e:tt;
		
		$($all:tt)*
	] => {
		$crate::append!(
			@[$($a)*]: @[$($b)*]: 
			$n - $e, __one_line_expr!($($all)*)
		)
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		$n:tt = $e:tt;
		
		$($all:tt)*
	] => {
		$crate::insert!(
			@[$($a)*]: @[$($b)*]:
			
			$n = [], append!(
				$n + $e, __one_line_expr!($($all)*)
			)
		)
	};
	/*[
		@[$($a:tt)*]: @[$($b:tt)*]:
		a = run( $n: ident ! );
		
		$($all:tt)*
	] => {
		$n ! {
			@[$($a)*]: @[$($b)*]:
			
			__one_line_expr!( #set(b, $($b)*); $($all)*  )
		}
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		b = run( $n: ident ! );
		
		$($all:tt)*
	] => {
		$n ! {
			@[$($a)*]: @[$($b)*]:
			
			__one_line_expr!( #set(a, $($a)*); $($all)*  )
		}
	};*/
	
	/*[
		@[$($a:tt)*]: @[$($b:tt)*]:
		run($n: ident !);
		
		$($all:tt)*
	] => {
		$n ! {
			@[$($a)*]: @[$($b)*]:
			
			__one_line_expr!( $($all)* )
		}
	};*/
	
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		dbg!($($args:tt)*);
		
		$($all:tt)*
	] => {
		$crate::debug_tree!( @[$($a)*]: @[$($b)*]: $($args)*, __one_line_expr!($($all)*))
	};
	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		dbg_num!($($args:tt)*);
		
		$($all:tt)*
	] => {
		$crate::debug_tree_nums!( @[$($a)*]: @[$($b)*]: $($args)*, __one_line_expr!($($all)*))
	};

	[
		@[$($a:tt)*]: @[$($b:tt)*]:
		# $var:ident = $($r:tt),*;
		
		$($all:tt)*
	] => {
		#[allow(unused_assignments)] {
			$var = $crate::into_num!( @[$($a)*]: @[$($b)*]: $($r),*);
		}
		
		$crate::__one_line_expr!( @[$($a)*]: @[$($b)*]: $($all)* )
	};
}

fn main() {
	let mut b = 0;
	one_line_expr!(
		a = 1;
		for a in 0..5.rev() {
			#b = a;
			
			#( println!("{:?}", b); );
		}
		#b = a;
		#( println!("end {:?}", b); );
	);
}
