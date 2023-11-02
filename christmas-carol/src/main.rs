// function to print the lyrics of the Twelve Days of Christmas
fn main() {
    twelve_days_of_christmas();
}

fn twelve_days_of_christmas() {
    // print the lyrics;

    for i in 1..=12 {
        match i {
            1 => println!("On the first day of Christmas, my true love gave to me"),
            2 => {
                println!("On the second day of Christmas, my true love gave to me");
                println!("A partridge in a pear tree");
            }
            3 => {
                println!("On the third day of Christmas, my true love gave to me");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            4 => {
                println!("On the fourth day of Christmas, my true love gave to me");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            5 => {
                println!("On the fifth day of Christmas, my true love gave to me");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            6 => {
                println!("On the sixth day of Christmas, my true love gave to me");
                println!("Five golden rings,");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            7 => {
                println!("On the seventh day of Christmas, my true love gave to me");
                println!("Six geese a-laying,");
                println!("Five golden rings,");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            8 => {
                println!("On the eighth day of Christmas, my true love gave to me");
                println!("Seven swans a-swimming,");
                println!("Six geese a-laying,");
                println!("Five golden rings,");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            9 => {
                println!("On the ninth day of Christmas, my true love gave to me");
                println!("Eight maids a-milking,");
                println!("Seven swans a-swimming,");
                println!("Six geese a-laying,");
                println!("Five golden rings,");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            10 => {
                println!("On the tenth day of Christmas, my true love gave to me");
                println!("Nine ladies dancing,");
                println!("Eight maids a-milking,");
                println!("Seven swans a-swimming,");
                println!("Six geese a-laying,");
                println!("Five golden rings,");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            11 => {
                println!("On the eleventh day of Christmas, my true love gave to me");
                println!("Ten lords a-leaping,");
                println!("Nine ladies dancing,");
                println!("Eight maids a-milking,");
                println!("Seven swans a-swimming,");
                println!("Six geese a-laying,");
                println!("Five golden rings,");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            12 => {
                println!("On the twelfth day of Christmas, my true love gave to me");
                println!("Twelve drummers drumming,");
                println!("Eleven pipers piping,");
                println!("Ten lords a-leaping,");
                println!("Nine ladies dancing,");
                println!("Eight maids a-milking,");
                println!("Seven swans a-swimming,");
                println!("Six geese a-laying,");
                println!("Five golden rings,");
                println!("Four calling birds,");
                println!("Three French hens,");
                println!("Two turtle doves, and");
                println!("A partridge in a pear tree");
            }
            _ => println!("Error"),
        }
    }
}
