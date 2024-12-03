declare const __dirname: string;

import * as fs from 'fs';
import * as path from 'path';

function parseLevels(levels: string[]) {
    const diffs = levels.reduce((acc: number[], level, index) => {
        if (index === 0) {
            return acc;
        }
        const prev = parseInt(levels[index - 1]);
        const current = parseInt(level);
        const diff = current - prev;
        return [...acc, diff];
    }
    , []);
    // check if all diffs are safe
    const diff_idx0 = Math.sign(diffs[0]);
    const isSafe =  diffs.reduce((isSafe: boolean, diff) => {
        if (!isSafe) {
            return false;
        }
        // return false if any diff is 0 or greater than 3
        // also return false if monotonicity is violated
        return isSafe && (Math.abs(diff) < 4) && (diff * diff_idx0 > 0);
    }, true);
    return isSafe;
}

function parseReports(lines: string[]) {
    const isSafeArray = lines.map((line) => {
        const levels = line.split(' ');
        if (line === '') {
            return false;
        }
        if (parseLevels(levels)) {
            return true;
        }
        for (let i = 0; i < levels.length; i++) {
            const levelsWithoutIth = levels.filter((level, index) => index !== i);
            const isSafe = parseLevels(levelsWithoutIth)
            if (isSafe) {
                return true;
            }
        }
        return false;
    });
    return isSafeArray.filter((isSafe) => isSafe).length;
}

const inputPath = path.join(__dirname, '../inputs/input.txt');
const input = fs.readFileSync(inputPath, 'utf-8');
const reports = input.split('\n');

const numSafeLevels = parseReports(reports);

console.log(`${numSafeLevels}`);