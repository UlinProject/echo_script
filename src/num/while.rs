
// TODO FIX __one_line_expr
#[macro_export]
macro_rules! r#while {
	[ @[$($a:tt)*]: @[$($b:tt)*]: while $n:tt $p:tt $v:tt {$($true:tt)*} else {$($end:tt)*}] => {
		$crate::r#if!(
			@[$($a)*]: @[$($b)*]:
			
			if $n $p $v {
				__one_line_expr!(
					$($true)* while $n $p $v {$($true)*} else {$($end)*}
				)
			}else {
				__one_line_expr!(
					$($end)*
				)
			}
		)
	};
}
