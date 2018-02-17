const Benchmark = require('benchmark');
const microtime = require('microtime');
const Database = require('better-sqlite3');

let times = [];
const iter = 20;

for (let i = 0; i < iter; i++) {
    const db = new Database('../../db.sqlite');
    const start = microtime.now()
    const row = db.prepare('SELECT * FROM users').all();
    JSON.stringify(row);
    const finish = microtime.now()
    times.push((finish - start) * 1000)
}

const timing = times.reduce(((p, c) => p + c), 0) / times.length

let res = [];
let moo = String(timing)
let counter = 0
for (let i = moo.length - 1; i >= 0; i--) {
    res.push(moo[i])
    counter++;
    if (counter % 3 === 0) {
        res.push('_')
    }
}

console.log(res.reverse().join('') + ' ns')
