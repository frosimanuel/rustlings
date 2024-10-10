// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn apple_price(apples:i32) -> i32 {

    if apples > 40 
    {
        apples
    } else {
        apples * 2
    }
}


fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(apple_price(35), 70);
        assert_eq!(apple_price(40), 80);
        assert_eq!(apple_price(41), 41);
        assert_eq!(apple_price(65), 65);
    }
}
