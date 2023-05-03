import fs from 'fs';
import csvParser from 'csv-parser';
import deasync from 'deasync';

// @ts-ignore
require.extensions['.txt'] = (module, filename) => {
    module.exports = fs.readFileSync(filename, 'utf8');
};

// @ts-ignore
require.extensions['.csv'] = (module, filename) => {
    const results: any[] = [];
    let isFinished = false;
    fs.createReadStream(filename).pipe(csvParser())
            .on('data', data => results.push(data))
            .on('end', () => isFinished = true);

    deasync.loopWhile(() => !isFinished);

    module.exports = results;
};