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
    values?: RegValueTypes
}

interface RegValueTypes {
    name: string,
    values: {
        [index: string]: OneRegValueType
    }
}

interface OneRegValueType {
    desc: string,
    name: string,
    value: string
}

interface Reg {
    name: string,
    addr: string,
    sections: RegSection[]
}

const data = _data as unknown as RegSection[];

const toComment = (string: string) => (string ?? '').split('\n').map(val => '/// ' + val).join('\n///\n');

const toCamelCase = (string: string) => {
    if (string.match(/^\d/)) {
        string = 'v' + string
    }

    const replacements = {
        fstx: 'FsTx',
        rxsingle: 'RxSingle',
        rxcontinuous: 'RxContinuous',
        fsrx: 'FsRx'
    }

    string = string
        .replace(/FSK\/OOK/g, 'FSK or OOK')
        .replace(/\//g, ' per ')
        .replace(/_/g, ' ')
        .split(/\s+/g)
        .map(word => {
            const lower = word.toLowerCase();
            if (['kHz', 'ms', 'us'].includes(word)) {
                return word;
            } else if (lower in replacements) {
                return (replacements as any)[lower];
            } else if (word === word.toUpperCase()) {
                return word.charAt(0) + lower.slice(1);
            } else {
                return word.charAt(0).toUpperCase() + word.slice(1);
            }
        }).join('')
        .replace(/[^0-9a-zA-Z]/g, '_')

    return string;
}

const getValueType = (section: RegSection) => {
    const valueTypes: RegValueTypes = {
        name: section.varName + 'Value',
        values: {}
    }

    section.loraDesc.split('\n')
            .filter(line => !line.toLowerCase().includes('reserved'))
            .forEach(line => {
                if (line.includes('-->')) {
                    const [binaryValue, desc] = line.split('-->').map(s => s.trim());
                    let name;

                    let insideParens;
                    if (insideParens = desc.match(/\(([A-Z0-9]*)\)/)) {
                        name = insideParens[1];
                    } else {
                        name = desc.replace(/\(.*\)/g, '').trim();
                    }

                    name = toCamelCase(name.split(/[,;=]/)[0].split(/\.[\sa-zA-Z]/)[0]);
                    
                    //console.log(name);

                    if (name == undefined) {
                        console.log(line);
                    }

                    valueTypes.values[name] = {
                        desc,
                        name,
                        value: '0b' + binaryValue
                    }
                }
            });

    return valueTypes;
}

let combined: Reg[] = data.reduce((arr: Reg[], curr: RegSection) => {
    const prev = arr[arr.length - 1];
    const prevSection = prev?.sections[prev?.sections.length - 1];
    if (!prev || curr.bits[0] == '7') {
        arr.push({
            name: curr.name,
            sections: [curr],
            addr: ''
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
}, [])
        .map(reg => ({ ...reg, name: reg.name.replace(/\s*/g, '')}))
        .map(reg => ({
            name: (reg.name.match(/^\w+/) ?? [])[0] || 'Reg' + reg.name.match(/\w+/)![0],
            addr: ((reg.name.match(/\((.*)\)/) ?? [])[1]).trim(),
            sections: reg.sections
                    .filter(section => !section.loraDesc.toLowerCase().includes('reserved') 
                            && !section.loraDesc.toLowerCase().includes('unused'))
                    .map(section => {
                        section.varName = section.varName.match(/\w*/)![0];
                        if (!section.varName) {
                            section.varName = toCamelCase(section.loraDesc);
                        }
                        const valueType = getValueType(section);

                        if (Object.keys(valueType.values).length > 0) {
                            section.values = valueType;
                            //console.log(Object.entries(section.values.values));
                        }

                        return section;
                    })
        }));

let string = `
// GENERATED FILE, DO NOT EDIT!

use crate::cant_hal::spi::devices::radio::radio_layout::*; 
`;

const finalized = combined
        .map(val => `
// Register
pub struct ${val.name};

impl RadioReg for ${val.name} {
    const ADDR: u8 = ${val.addr};
}` + val.sections.map(section => {
    const regPart = val.name + section.varName;

    return `
// Register part
${toComment(section.loraDesc)}
pub struct ${regPart};

impl RadioRegPart for ${regPart} {
    type Reg = ${val.name};
    type Value = ${section.values?.name ?? 'u8'};
    const WRITABLE: bool = ${section.mode.includes('w')};
    const READABLE: bool = ${section.mode.includes('r')};
    const TRIGGERABLE: bool = ${section.mode.includes('t')};
    const CLEARABLE: bool = ${section.mode.includes('c')};
    const START: u8 = ${section.bits[0]};
    const END: u8 = ${section.bits.split(/\-\s*/)[1] || section.bits[0]};
}


${section.values ? `// Register enum
pub enum ${section.values.name} {
        ${Object.values(section.values.values).map((valueType: OneRegValueType) => `
        ${toComment(valueType.desc)}
        ${valueType.name},
    `).join('')}}

impl RadioRegPartValue for ${section.values.name} {
    fn value(&self) -> u8 {
        match self {
${Object.values(section.values.values).map((valueType: OneRegValueType) => 
`           Self::${valueType.name} => ${valueType.value}`).join(',\n')}
        }
    }
}` : ''}`}).join('')).join('').replace(/\&amp;/g, '&');


string += finalized;

fs.writeFileSync(__dirname + '/../../reg_gen.rs', string);
fs.writeFileSync(__dirname + '/combined.json', JSON.stringify(combined, null, 4));


//console.log(util.inspect(combined, { depth: null, colors: true }));