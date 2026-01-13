// @ts-check

/**
 * Generates a random starship registry number.
 *
 * @returns {string} the generated registry number.
 */
export function randomShipRegistryNumber() {
  const number = Math.floor(Math.random() * (9999 - 1000) + 1000);
  console.log(`NCC-${number}`);
  return `NCC-${number}`;
}

/**
 * Generates a random stardate.
 *
 * @returns {number} a stardate between 41000 (inclusive) and 42000 (exclusive).
 */
export function randomStardate() {
  console.log("ERROR HERE")
  return Math.random() * (42000 - 41000) + 41000;
}

/**
 * Generates a random planet class.
 *
 * @returns {string} a one-letter planet class.
 */
export function randomPlanetClass() {
  const arrayOfPlanetClass = ['D', 'H', 'J', 'K', 'L', 'M', 'N', 'R', 'T', 'Y'];
  const randomNumber = Math.floor(Math.random() * (arrayOfPlanetClass.length));
 
  return arrayOfPlanetClass[randomNumber];
}
