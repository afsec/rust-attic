/**
* Gets random int
* @param min 
* @param max 
* @returns random int - min & max inclusive
*/
export function getRandomInt(min: number, max: number): number {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

export function getShuffledArray(): Array<number> {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let shuffled = [];
    let currentArrLength = arr.length;
    while (currentArrLength > 0) {
        const idx = getRandomInt(0, currentArrLength - 1);
        currentArrLength -= 1;
        const x = arr.splice(idx, 1).pop();
        shuffled.push(x);
    }

    return shuffled

}