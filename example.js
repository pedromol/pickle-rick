const pickleRick = require(".");
const { promisify } = require('util');

const unpickle = promisify(pickleRick.unpickle);

const example = async () => {
    console.log('example.pickle');
    const example = await unpickle('./example.pickle');
    console.log(example);

    console.log('dict.pickle');
    const dict = await unpickle('./dict.pickle');
    console.log(dict);

    console.log('err handling');
    try {
        await unpickle('./README.md');
    } catch (err) {
        console.log(err);
    }
};

example();
