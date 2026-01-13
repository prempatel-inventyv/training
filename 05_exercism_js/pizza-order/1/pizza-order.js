/// <reference path="./global.d.ts" />
//
// @ts-check

/**
 * Determine the price of the pizza given the pizza and optional extras
 *
 * @param {Pizza} pizza name of the pizza to be made
 * @param {Extra[]} extras list of extras
 *
 * @returns {number} the price of the pizza
 */
export function pizzaPrice(pizza, ...extras) {
  let pizzaPri = 0;
  const basePrices = {
    Margherita: 7,
    Caprese: 9,
    Formaggio: 10,
  };
  // switch(pizza){
  //   case "Margherita":
  //     pizzaPri = 7;
  //     break;
  //   case "Caprese":
  //     pizzaPri = 9;
  //     break;
  //   case "Formaggio":
  //     pizzaPri = 10
  //     break
  // }
  function calculateExtra(...extra){
    let price = 0;
    const extraPrices = {
      ExtraSauce: 1,
      ExtraToppings: 2,
    };
    if(extra.length === 0)
      return 0;

    const [firstThing,...remaining] = extra;
    price += extraPrices[firstThing]
    console.log(remaining.length);
    return price + calculateExtra(...remaining);
  }
  const extraPay = calculateExtra(...extras);
  return basePrices[pizza] + extraPay;
}

/**
 * Calculate the price of the total order, given individual orders
 *
 * (HINT: For this exercise, you can take a look at the supplied "global.d.ts" file
 * for a more info about the type definitions used)
 *
 * @param {PizzaOrder[]} pizzaOrders a list of pizza orders
 * @returns {number} the price of the total order
 */
export function orderPrice(pizzaOrders) {
  const basePrices = {
    Margherita: 7,
    Caprese: 9,
    Formaggio: 10,
  };

  const extraPrices = {
    ExtraSauce: 1,
    ExtraToppings: 2,
  };

  return pizzaOrders.reduce((total, order) => {
    total += basePrices[order.pizza];

    for (const extra of order.extras) {
      total += extraPrices[extra];
    }

    return total;
  }, 0);
}

