// Run like this:
// $ ts-node ./scripts/gen.ts

import './common';
import fs from 'fs/promises';
import recursiveReaddir from 'recursive-readdir';

(async () => {
    const files = await recursiveReaddir('src');

    files.map(fileName => {
        if (fileName.includes('gen.ts')) {
            require('../' + fileName.split('.ts')[0]);
        }
    })
})();