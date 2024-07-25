
use tttest::one_line_expr;

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
