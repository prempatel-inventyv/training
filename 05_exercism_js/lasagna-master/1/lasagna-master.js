/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */

export function cookingStatus(time) {
  console.log(time)
  if(time === 0) return 'Lasagna is done.';
  else if(!time) return "You forgot to set the timer.";
  else return "Not done, please wait.";
}

export function preparationTime(layers,time=2) {
  const length = layers.length;
  return length * time;
}

export function quantities(quantities) {
  let noodles = 0;
  let sauce = 0;
  quantities.forEach((data) => {
  if (data === "sauce") {
    sauce++;
  } else if (data === "noodles") {
    noodles++;
  }
});
  return {
    noodles : noodles * 50,
    sauce : sauce * 0.2
  }
}

export function addSecretIngredient(friendsList,myList) {
  const len = friendsList.length;
  const secretIngredient = friendsList[len-1];
  myList.push(secretIngredient);
}

export function scaleRecipe(recipe,portions) {
  let newObj = {}
  for(let key in recipe){
    newObj[key] = (recipe[key] * portions) / 2;
  }
  console.log(newObj)
  return newObj;
}


