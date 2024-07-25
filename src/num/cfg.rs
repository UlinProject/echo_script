
#[macro_export]
macro_rules! r#if {
	[ // A a == 0
		@[]:
		@[$($b:tt)*]:
		
		if a == 0 {
			$n: ident ! $(($($args:tt)*))?
		}else {
			$($_unk:tt)*
		}
	] => {
		$n! {
			@[]:@[$($b)*]: $($($args)*)?
		}
	};
	[ // A a == 0
		@[ $([$r:tt])+ ]:
		@[$($b:tt)*]:
		
		if a == 0 {
			$($_unk:tt)*
		}else {
			$n: ident ! $(($($args:tt)*))?
		}
	] => {
		$n! {
			@[ $([$r])+ ]: @[$($b)*]: $($($args)*)?
		}
	};
	[ // A a == 0
		@[]:
		@[$($b:tt)*]:
		
		if a != 0 {
			$($_unk:tt)*
		}else {
			$n: ident ! $(($($args:tt)*))?
		}
	] => {
		$n! {
			@[]:@[$($b)*]: $($($args)*)?
		}
	};
	[ // A a == 0
		@[ $([$r:tt])+ ]:
		@[$($b:tt)*]:
		
		if a != 0 {
			$n: ident ! $(($($args:tt)*))?
		}else {
			$($_unk:tt)*
		}
	] => {
		$n! {
			@[ $([$r])+ ]: @[$($b)*]: $($($args)*)?
		}
	};
	
	[  // B b == 0
		@[$($a:tt)*]:
		@[]:
		
		if b == 0 {
			$n: ident ! $(($($args:tt)*))?
		}else {
			$($_unk:tt)*
		}
	] => {
		$n! {
			@[$($a)*]: @[]: $($($args)*)?
		}
	};
	[ // B b == 0
		@[ $($a:tt)* ]:
		@[ $([$r:tt])+ ]:
		
		if b == 0 {
			$($_unk:tt)*
		}else {
			$n: ident ! $(($($args:tt)*))?
		}
	] => {
		$n! {
			@[$($a)*]:@[ $([$r])+ ]: $($($args)*)?
		}
	};
	[  // B
		@[$($a:tt)*]:
		@[]:
		
		if b != 0 {
			$($_unk:tt)*
		}else {
			$n: ident ! $(($($args:tt)*))?
		}
	] => {
		$n! {
			@[$($a)*]: @[]: $($($args)*)?
		}
	};
	[ // B
		@[ $($a:tt)* ]:
		@[ $([$r:tt])+ ]:
		
		if b != 0 {
			$n: ident ! $(($($args:tt)*))?
		}else {
			$($_unk:tt)*
		}
	] => {
		$n! {
			@[$($a)*]:@[ $([$r])+ ]: $($($args)*)?
		}
	};
}
