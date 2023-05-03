import fs from 'fs';
import _data from './raw.csv';
import util from 'util';

interface RegSection {
    name: string,
    bits: string,
    varName: string,
    mode: string,
    reset: string,
    loraDesc: string
}

interface Reg {
    name: string,
    sections: RegSection[]
}

const data = _data as unknown as RegSection[];

//console.log(fs.readFileSync(__dirname + '/raw.csv').toString());
let combined = data.reduce((arr: Reg[], curr: RegSection) => {
    const prev = arr[arr.length - 1];
    const prevSection = prev?.sections[prev?.sections.length - 1];
    if (!prev || curr.bits[0] == '7') {
        arr.push({
            name: curr.name,
            sections: [curr]
        });
    } else {
        prev.name += curr.name;

        if (!curr.bits) {
            prevSection.varName += curr.varName;
            prevSection.loraDesc += '\n' + curr.loraDesc;
        } else {
            prev.sections.push(curr)
        }
    }

    return arr;
}, []);

console.log(util.inspect(combined, { depth: null, colors: true }));