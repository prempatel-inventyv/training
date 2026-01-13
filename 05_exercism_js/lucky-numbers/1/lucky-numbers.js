// @ts-check

/**
 * Calculates the sum of the two input arrays.
 *
 * @param {number[]} array1
 * @param {number[]} array2
 * @returns {number} sum of the two arrays
 */

function getAllElement(arr) {
  let ans = "";
  for(let i in arr){
    ans += arr[i]
  }
  return ans;
}

export function twoSum(array1, array2) {
  const ans1 = getAllElement(array1);
  const ans2 = getAllElement(array2);

  return Number(ans1) + Number(ans2);
}

/**
 * Checks whether a number is a palindrome.
 *
 * @param {number} value
 * @returns {boolean} whether the number is a palindrome or not
 */
export function luckyNumber(value) {
  const str = String(value);
  let i = 0;
  let j = str.length - 1;
  while(i<j){
    if(str[i] !== str[j])
      return false;
    i += 1;
    j -= 1;
  }
  return true;
}

/**
 * Determines the error message that should be shown to the user
 * for the given input value.
 *
 * @param {string|null|undefined} input
 * @returns {string} error message
 */
export function errorMessage(input) {
  if (input === null || input === undefined || input === '') {
    return 'Required field';
  }

  const number = Number(input);

  if (number === 0 || Number.isNaN(number)) {
    return 'Must be a number besides 0';
  }

  return '';
}

