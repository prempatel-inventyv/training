// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Determines how long it takes to prepare a certain juice.
 *
 * @param {string} name
 * @returns {number} time in minutes
 */
export function timeToMixJuice(name) {
  let timeInMinutes = 0;
  switch(name){
    case "Pure Strawberry Joy":
        timeInMinutes = 0.5;
        break;
    case "Energizer":
        timeInMinutes = 1.5;
        break;
    case "Green Garden":
        timeInMinutes = 1.5;
        break;
    case "Tropical Island":
        timeInMinutes = 3;
        break; 
    case "All or Nothing":
        timeInMinutes = 5;
        break;  
    default:
        timeInMinutes = 2.5
  }
      return timeInMinutes;
}

/**
 * Calculates the number of limes that need to be cut
 * to reach a certain supply.
 *
 * @param {number} wedgesNeeded
 * @param {string[]} limes
 * @returns {number} number of limes cut
 */
export function limesToCut(wedgesNeeded, limes) {
  let totalLimes = 0;
  let index = 0;
  while (totalLimes < wedgesNeeded && index < limes.length) {
    const name = limes[index];
    switch (name) {
      case "small":
        totalLimes += 6;
        break;
      case "medium":
        totalLimes += 8;
        break;
      case "large":
        totalLimes += 10;
        break;
    }
    index += 1;
  }

  return index;
}


/**
 * Determines which juices still need to be prepared after the end of the shift.
 *
 * @param {number} timeLeft
 * @param {string[]} orders
 * @returns {string[]} remaining orders after the time is up
 */
export function remainingOrders(timeLeft, orders) {
  let index = 0;
  console.log(orders)
  console.log("TIME LEFT :",timeLeft);
  do{
    const juice = orders[index];
    const time = timeToMixJuice(juice);
    timeLeft = timeLeft - time;
     console.log({
      juice,
      index,
      time,
      timeLeft
    })
    orders.shift();
  }while(timeLeft > 0)
  console.log("ORDERS LAST :",orders);
  return orders;
}
