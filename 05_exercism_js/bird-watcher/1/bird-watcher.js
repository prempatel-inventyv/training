// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Calculates the total bird count.
 *
 * @param {number[]} birdsPerDay
 * @returns {number} total bird count
 */
export function totalBirdCount(birdsPerDay) {
  let ans = 0;
  const len = birdsPerDay.length;
  for(let i = 0;i < len; i++){
    ans += birdsPerDay[i]
  }
  return ans;
}

/**
 * Calculates the total number of birds seen in a specific week.
 *
 * @param {number[]} birdsPerDay
 * @param {number} week
 * @returns {number} birds counted in the given week
 */
export function birdsInWeek(birdsPerDay, week) {
  console.log(birdsPerDay,week)
  let ans = 0;
  const start = (week - 1) * 7;
  for(let i = start;i<start+7;i++){
    ans+=birdsPerDay[i]
  }
  return ans;
}

/**
 * Fixes the counting mistake by increasing the bird count
 * by one for every second day.
 *
 * @param {number[]} birdsPerDay
 * @returns {void} should not return anything
 */
export function fixBirdCountLog(birdsPerDay) {
  for(let i=0;i<birdsPerDay.length;i++){
    if(i%2==0) birdsPerDay[i] = birdsPerDay[i] + 1;
  }
  return birdsPerDay;
}
