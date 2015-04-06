// MALBOLGE 언어 인터프리터
//
// C로 작성된 MALBOLGE의 인터프리터를 Rust 1.0 Beta로 다시 작성함.
// 원본코드는 다음 주소에서 얻을 수 있습니다:
//    http://www.lscheffer.com/malbolge_interp.html
//
// 테스트를 위한 코드는 다음 주소에서 얻을 수 있습니다:
//  * Hello, World       : http://acooke.org/malbolge.html
//  * 99 Bottles of Beer : http://www.99-bottles-of-beer.net/language-malbolge-995.html
//
// 아래는 원본 코드의 주석입니다.
//
/* Interpreter for Malbolge.                                          */
/* '98 Ben Olmstead.                                                  */
/*                                                                    */
/* Malbolge is the name of Dante's Eighth circle of Hell.  This       */
/* interpreter isn't even Copylefted; I hereby place it in the public */
/* domain.  Have fun...                                               */
/*                                                                    */
/* Note: in keeping with the idea that programming in Malbolge is     */
/* meant to be hell, there is no debugger.                            */
/*                                                                    */
/* By the way, this code assumes that short is 16 bits.  I haven't    */
/* seen any case where it isn't, but it might happen.  If short is    */
/* longer than 16 bits, it will still work, though it will take up    */
/* considerably more memory.                                          */
/*                                                                    */
/* If you are compiling with a 16-bit Intel compiler, you will need   */
/* >64K data arrays; this means using the HUGE memory model on most   */
/* compilers, but MS C, as of 8.00, possibly earlier as well, allows  */
/* you to specify a custom memory-model; the best model to choose in  */
/* this case is /Ashd (near code, huge data), I think.                */

use std::char;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{ Read, stdin };

// Checks whether c is a white-space character.
// White-space characters are any of
// ' '  (0x20) Space (SPC)
// '\t' (0x09) Horizontal Tab (TAB)
// '\n' (0x0A) Newline (LF)
// '\v' (0x0B) Vertical Tab(VT)
// '\f' (0x0C) Feed (FF)
// '\r' (0x0D) Carrage Return (CR)
fn isspace( c: u8 ) -> bool
{
	return match c
	{
		0x20  => true,
		0x09 => true,
		0x0a => true,
		0x0b => true,
		0x0c => true,
		0x0d => true,
		_    => false,
	};
}


static XLAT1 : [char;94] =[
	'+', 'b', '(', '2', '9', 'e', '*', 'j', '1', 'V',
	'M', 'E', 'K', 'L', 'y', 'C', '}', ')', '8', '&',
	'm', '#', '~', 'W', '>', 'q', 'x', 'd', 'R', 'p',
	'0', 'w', 'k', 'r', 'U', 'o', '[', 'D', '7', ',',
	'X', 'T', 'c', 'A', '"', 'l', 'I', '.', 'v', '%',
	'{', 'g', 'J', 'h', '4', 'G', '\\', '-', '=', 'O',
	'@', '5', '`', '_', '3', 'i', '<', '?', 'Z', '\'',
	';', 'F', 'N', 'Q', 'u', 'Y', ']', 's', 'z', 'f',
	'$', '!', 'B', 'S', '/', '|', 't', ':', 'P', 'n',
	'6', '^', 'H', 'a' ];

static XLAT2 : [char;94] = [
	'5', 'z', ']', '&', 'g', 'q', 't', 'y', 'f', 'r',
	'$', '(', 'w', 'e', '4', '{', 'W', 'P', ')', 'H',
	'-', 'Z', 'n', ',', '[', '%', '\\', '3', 'd', 'L',
	'+', 'Q', ';', '>', 'U', '!', 'p', 'J', 'S', '7',
	'2', 'F', 'h', 'O', 'A', '1', 'C', 'B', '6', 'v',
	'^', '=', 'I', '_', '0', '/', '8', '|', 'j', 's',
	'b', '9', 'm', '<', '.', 'T', 'V', 'a', 'c', '`',
	'u', 'Y', '*', 'M', 'K', '\'', 'X', '~', 'x', 'D',
	'l', '}', 'R', 'E', 'o', 'k', 'N', ':', '#', '?',
	'G', '"', 'i', '@' ];


// Malbolge의 명령어인지 검사.
// 원본 C 코드에서
// strchr( "ji*p</vo", xlat1[( x - 33 + i ) % 94] ) == NULL
// 부분을 대체하기 위해 사용한다.
fn valid_op( o: char ) -> bool
{
	match o
	{
		'j' => true,
		'i' => true,
		'*' => true,
		'p' => true,
		'<' => true,
		'/' => true,
		'v' => true,
		'o' => true,
		_    => false
	}
}

fn main()
{
	let args: Vec<_> = env::args().collect();
	if args.len() != 2
	{
		println!( "invalid command line" );
		return;
	}

	let f_path = Path::new( &args[1] );
	let f = match File::open( f_path )
	{
		Ok(f) => f,
		Err(_) => {
			 println!( "can't open file." );
			 return;
		}
	};

	let mut i : usize = 0;
	//let mut j : u16;
	let mut x : u8;
	let mut mem : [u16; 59049] = [0; 59049];

	let mut src_bytes = f.bytes();
	loop
	{
		x = match src_bytes.next()
		{
			Some(r) => r.unwrap(),
			None => break,
		};

		if isspace(x)
		{
			continue;
		}

		if x < 127 && x > 32
		{
			let _op = XLAT1[ (x as usize - 33 + i) % 94 ];
			if !valid_op( _op )
			{
				println!( "invalid character in source file" );
				return;
			}
		}

		if i == 59049
		{
			println!( "input file too long" );
			return;
		}

		mem[i] = x as u16;
		i += 1;
	}

	while i < 59049
	{
		mem[i] = op( mem[i - 1], mem[i - 2] );
		i += 1;
	}
	exec( &mut mem );
}

fn exec( mem: &mut [u16; 59049] )
{
	let mut a: u16 = 0;
	let mut c: u16 = 0;
	let mut d: u16 = 0;

	loop
	{
		if mem[c as usize] < 33 || mem[c as usize] > 126
		{
			continue;
		}

		let o = XLAT1[((mem[c as usize] - 33 + c) % 94) as usize];
		match o
		{
			'j' => d = mem[d as usize],
			'i' => c = mem[d as usize],
			'*' => {
				let _mem_d = mem[d as usize];
				mem[d as usize] = _mem_d / 3 + _mem_d % 3 * 19683;
				a = mem[d as usize]
			},
			'p' => {
				mem[d as usize] = op( a, mem[d as usize] );
				a = mem[d as usize];
			},
			'<' => {
				// C언어의 putc()와 동일한 출력을 위해
				// 하위 1바이트만 남긴다.
				let _a_ch: u32 = (a & 0x000000FF) as u32;
				print!( "{}", char::from_u32(_a_ch).unwrap() )
			},
			'/' => {
				// C의 getc()를 사용한 본래의 인터프리터와는 달리
				// 입력시 반드시 return key를 눌러주어야 한다.
				// rust에서 getc()를 사용할 수 있을까?
				let x = read_single_byte();

				// EOF or Ctrl+Z
				// TODO: Ctrl+Z에 대한 입력은 임의로 추가하였음.
				if x == -1 || x == 26
				{
					a = 59048;
				}
				else
				{
					a = x as u16;
				}
			}
			'v' => return,
			_ => {}
		}

		mem[c as usize] = XLAT2[(mem[c as usize] -33) as usize] as u16;
		if c == 59048
		{
			c = 0;
		}
		else
		{
			c += 1;
		}

		if d == 59048
		{
			d = 0;
		}
		else
		{
			d += 1;
		}
	}
}

static OP_P9: [u16;5] = [ 1, 9, 81, 729, 6561 ];
static OP_O: [ [u16;9]; 9 ] =
[
	[ 4, 3, 3, 1, 0, 0, 1, 0, 0 ],
	[ 4, 3, 5, 1, 0, 2, 1, 0, 2 ],
	[ 5, 5, 4, 2, 2, 1, 2, 2, 1 ],
	[ 4, 3, 3, 1, 0, 0, 7, 6, 6 ],
	[ 4, 3, 5, 1, 0, 2, 7, 6, 8 ],
	[ 5, 5, 4, 2, 2, 1, 8, 8, 7 ],
	[ 7, 6, 6, 7, 6, 6, 4, 3, 3 ],
	[ 7, 6, 8, 7, 6, 8, 4, 3, 5 ],
	[ 8, 8, 7, 8, 8, 7, 5, 5, 4 ],
];

fn op( x: u16, y: u16 ) -> u16
{
	let mut i: u16 = 0;

	for j in 0..5
	{
		let y_index: usize = (y / OP_P9[j] % 9) as usize;
		let x_index: usize = (x / OP_P9[j] % 9) as usize;

		i += OP_O[ y_index ][ x_index ] * OP_P9[j];
	}

	return i;
}

///
/// 표준입력(stdin)에서 한 바이트를 읽는다.
///
fn read_single_byte() -> i8
{
	let mut s = String::new();
	let len = stdin().read_line(&mut s);

	if len.unwrap() >= 3
	{
		s.as_bytes()[ 0 ] as i8
	}
	else
	{
		0
	}
}

