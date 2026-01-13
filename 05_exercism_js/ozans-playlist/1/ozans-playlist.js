// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Removes duplicate tracks from a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} new playlist with unique entries
 */
export function removeDuplicates(playlist) {
  const rtnArr = new Set(playlist);
  return Array.from(rtnArr);
}

/**
 * Checks whether a playlist includes a track.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {boolean} whether the track is in the playlist
 */
export function hasTrack(playlist, track) {
  const newSet = new Set(playlist);
  const len = newSet.size;
  newSet.add(track);
  const newLen = newSet.size;
  console.log({len,newLen,playlist,track})
  return len === newLen ? true : false;
}

/**
 * Adds a track to a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function addTrack(playlist, track) {
  const newSet = new Set(playlist);
  newSet.add(track);
  return Array.from(newSet);
}

/**
 * Deletes a track from a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function deleteTrack(playlist, track) {
  const newArr = playlist.filter(data => data !== track && data)
  console.log({newArr,playlist, track})
  return newArr;
}

/**
 * Lists the unique artists in a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} list of artists
 */
export function listArtists(playlist) {
  const newArr = [];
  for (let track of playlist) {
    newArr.push(track.split('- ')[1])
  }
  const set = new Set(newArr);
  return Array.from(set);
}
