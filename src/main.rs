// The default values inside this project are for the BMW 2 series Gran Coupe with prices of Turkey.
// And before I get bashed for not using python please shut up as I am trying to improve my rust code.

/*
 * The price of gas and diesel at the current moment inside the city you are living in.
 * I am not using an API cuz they ain't free and I ain't got money for that lmao.
 */
const GAS_PRICE: f32 = 19.0;
const DIESEL_PRICE: f32 = 21.22;

/*
 * How many liters of the given fuel is required to move
 * the car 100 km inside an average city environment.
 */
const GAS_PER_100_KM: f32 = 7.2;
const DIESEL_PER_100_KM: f32 = 5.0;

/*
 * The prices of the diesel version and the gas version of the car you want to buy.
 * Make sure to use accurate values and not round any price.
 */
const GAS_CAR_PRICE: f32 = 967694.4;
const DIESEL_CAR_PRICE: f32 = 994244.4;

fn main() {
    // The amount of money you are paying in order to go 1 km.
    let gas = (GAS_PER_100_KM / 100.0) * GAS_PRICE;
    let diesel = (DIESEL_PER_100_KM / 100.0) * DIESEL_PRICE;

    // The difference the prices of the vehicles and the difference between the L/km.
    let fuel_difference = gas - diesel;
    let car_difference = DIESEL_CAR_PRICE - GAS_CAR_PRICE;

    // This calculates how many kilometers is required to profit from buying a diesel car
    // by diving the price difference of the cars by the profit from the fuel.
    let result = car_difference / fuel_difference;

    // If the result is above 0 this means you will profit by buying a diesel vehicle.
    if result > 0.0 {
        println!("You would need to drive {} kilometers in order to be actually profiting from a diesel car.", result);
        println!("Keep in mind this does not take maintenance fees into account and diesel is generally more expensive!");
    }

    // If the result is 0 that means it doesn't matter if you buy a diesel other than maintenance fees.
    else if result == 0.0 {
        println!("It's equal! You are free to choose whichever you want!");
        println!("Keep in mind this does not take maintenance into account and diesel is generally more expensive!");
    }

    // If the result is negative, this means you can never profit from buying the diesel option.
    else {
        println!("You cannot profit from buying a diesel car with the current prices in the long run.");
    }
}
