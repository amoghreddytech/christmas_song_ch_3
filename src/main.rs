fn main() {

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth","tenth", "eleventh","twelfth"];
    
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
        ];

    
    let mut index = 1;

    println!("------------------------------------------------------------------------");
    for day in days {
        println!("On the {day} day of Christmas, my true love sent to me");
        
        
        for j in (0..index).rev(){
            println!("{}",lyrics[j].trim());
        };

        index += 1;
        println!("-----------------------------------------------------------------------");
    };

    
}
