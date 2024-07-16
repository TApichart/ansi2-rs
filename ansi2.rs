#![allow(warnings)]

const ESCs:char = '\x1b';
const RST:&str = "\x1b[0m";  // RESET
const ITLC:&str = "\x1b[3m";  // Italic
const BLIK:&str = "\x1b[5m";  // Blink

enum ANSICOLOR { BLK, RED, GRN, YEL, BLU, MAG, CYN, WHT }
enum ANSIMODE  { RST, BLD, DIM, ILC, UDL, BLI, REV, INV, STK }


fn BFcolor (bg:ANSICOLOR, fg:ANSICOLOR) -> String {
	String::from(format! ("\x1b[4{};3{}m", bg as u8, fg as u8 ))
}


fn BrightBF (bg:ANSICOLOR, fg:ANSICOLOR) -> String {
    String::from(format! ("\x1b[10{};9{}m", bg as u8, fg as u8 ))
}


fn BlinkMsg (msg:String, bg:ANSICOLOR, fg:ANSICOLOR) -> String {
    String::from(format! ("\x1b[5;10{};9{}m{}\x1b[25m", bg as u8, fg as u8, msg) )
}


fn show256() {
    let mut cid:u8;
    let mut cmod:u8;
    for cid in 0..=255 {
        cmod = cid % 10;
        print! ("\x1b[40;38;5;{}m{} \x1b[0m", cid, cid)
    }
}


fn testMode() {
    let mut ansimode:u8;
    let textstyle:String = BFcolor(ANSICOLOR::WHT, ANSICOLOR::RED);
    for ansimode in 0..=9 {
        println! ("\x1b[{}m{} TEXT STYLE ... ANSI MODE [{}] {}", ansimode, textstyle, ansimode, RST)
    }
}


fn main() {
	print! ("{}{}RUST Programming{}", ITLC, 
		BFcolor (ANSICOLOR::RED, ANSICOLOR::WHT), RST );
	println! ("{}{} Language...{}", BLIK, 
		BFcolor (ANSICOLOR::BLU, ANSICOLOR::WHT), RST );
	
    print! ("{}{}RUST Programming{}", ITLC, 
		BrightBF (ANSICOLOR::RED, ANSICOLOR::WHT), RST );
	println! ("{}{} Language...{}", BLIK, 
		BrightBF (ANSICOLOR::BLU, ANSICOLOR::WHT), RST );

    println! ( "{}", BlinkMsg (String::from("BLINK..blink..Messages..."), ANSICOLOR::WHT, ANSICOLOR::MAG) );
    show256 ();
    println! ("");
    testMode ();
    println! ( "{}", RST )
}
