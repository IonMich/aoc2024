"use strict";
var __spreadArray = (this && this.__spreadArray) || function (to, from, pack) {
    if (pack || arguments.length === 2) for (var i = 0, l = from.length, ar; i < l; i++) {
        if (ar || !(i in from)) {
            if (!ar) ar = Array.prototype.slice.call(from, 0, i);
            ar[i] = from[i];
        }
    }
    return to.concat(ar || Array.prototype.slice.call(from));
};
Object.defineProperty(exports, "__esModule", { value: true });
var fs = require("fs");
var path = require("path");
function parseLevels(levels) {
    var diffs = levels.reduce(function (acc, level, index) {
        if (index === 0) {
            return acc;
        }
        var prev = parseInt(levels[index - 1]);
        var current = parseInt(level);
        var diff = current - prev;
        return __spreadArray(__spreadArray([], acc, true), [diff], false);
    }, []);
    // check if all diffs are safe
    var diff_idx0 = Math.sign(diffs[0]);
    var isSafe = diffs.reduce(function (isSafe, diff) {
        if (!isSafe) {
            return false;
        }
        // return false if any diff is 0 or greater than 3
        // also return false if monotonicity is violated
        return isSafe && (Math.abs(diff) < 4) && (diff * diff_idx0 > 0);
    }, true);
    return isSafe;
}
function parseReports(lines) {
    var isSafeArray = lines.map(function (line) {
        var levels = line.split(' ');
        if (line === '') {
            return false;
        }
        if (parseLevels(levels)) {
            return true;
        }
        var _loop_1 = function (i) {
            var levelsWithoutIth = levels.filter(function (level, index) { return index !== i; });
            var isSafe = parseLevels(levelsWithoutIth);
            if (isSafe) {
                return { value: true };
            }
        };
        for (var i = 0; i < levels.length; i++) {
            var state_1 = _loop_1(i);
            if (typeof state_1 === "object")
                return state_1.value;
        }
        return false;
    });
    return isSafeArray.filter(function (isSafe) { return isSafe; }).length;
}
var inputPath = path.join(__dirname, '../inputs/input.txt');
var input = fs.readFileSync(inputPath, 'utf-8');
var reports = input.split('\n');
var numSafeLevels = parseReports(reports);
console.log("".concat(numSafeLevels));
